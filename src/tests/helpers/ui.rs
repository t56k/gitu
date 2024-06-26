use crate::{
    cli::Args,
    config,
    key_parser::parse_keys,
    state::State,
    term::{Term, TermBackend},
    tests::helpers::RepoTestContext,
};
use crossterm::event::{Event, KeyEvent};
use git2::Repository;
use ratatui::{backend::TestBackend, prelude::Rect, Terminal};
use std::path::PathBuf;
use temp_dir::TempDir;

use self::buffer::TestBuffer;

mod buffer;

pub struct TestContext {
    pub term: Term,
    pub dir: TempDir,
    pub remote_dir: TempDir,
    pub size: Rect,
}

impl TestContext {
    pub fn setup_init(width: u16, height: u16) -> Self {
        let term = Terminal::new(TermBackend::Test(TestBackend::new(width, height))).unwrap();
        let repo_ctx = RepoTestContext::setup_init();

        Self {
            term,
            dir: repo_ctx.dir,
            remote_dir: repo_ctx.remote_dir,
            size: Rect::new(0, 0, width, height),
        }
    }

    pub fn setup_clone(width: u16, height: u16) -> Self {
        let term = Terminal::new(TermBackend::Test(TestBackend::new(width, height))).unwrap();
        let repo_ctx = RepoTestContext::setup_clone();

        Self {
            term,
            dir: repo_ctx.dir,
            remote_dir: repo_ctx.remote_dir,
            size: Rect::new(0, 0, width, height),
        }
    }

    pub fn init_state(&mut self) -> State {
        self.init_state_at_path(self.dir.path().to_path_buf())
    }

    pub fn init_state_at_path(&mut self, path: PathBuf) -> State {
        let mut state = State::create(
            Repository::open(path).unwrap(),
            self.size,
            &Args::default(),
            config::init_test_config().unwrap(),
            false,
        )
        .unwrap();

        // hack: Pass in an event just to force re-rendering
        state.update(&mut self.term, &[Event::FocusGained]).unwrap();
        state
    }

    pub fn redact_buffer(&self) -> String {
        let TermBackend::Test(test_backend) = self.term.backend() else {
            unreachable!();
        };
        let mut debug_output = format!("{:?}", TestBuffer(test_backend.buffer()));

        [&self.dir, &self.remote_dir]
            .iter()
            .flat_map(|dir| Repository::open(dir.path()).ok())
            .for_each(|repo| {
                let mut revwalk = repo.revwalk().unwrap();
                revwalk.push_head().ok();
                revwalk
                    .flat_map(|maybe_oid| maybe_oid.and_then(|oid| repo.find_commit(oid)))
                    .for_each(|commit| {
                        let id = commit.as_object().id().to_string();
                        let short = commit.as_object().short_id().unwrap();
                        let short_id = short.as_str().unwrap();

                        debug_output = debug_output.replace(&id, &"_".repeat(id.len()));
                        debug_output = debug_output.replace(short_id, &"_".repeat(short_id.len()));
                    });
            });

        redact_temp_dir(&self.dir, &mut debug_output);
        redact_temp_dir(&self.remote_dir, &mut debug_output);

        debug_output
    }
}

fn redact_temp_dir(temp_dir: &TempDir, debug_output: &mut String) {
    let text = temp_dir.path().to_str().unwrap();
    *debug_output = debug_output.replace(text, &" ".repeat(text.len()));
}

pub fn keys(input: &str) -> Vec<Event> {
    let ("", keys) = parse_keys(input).unwrap() else {
        unreachable!();
    };

    keys.into_iter()
        .map(|(mods, key)| Event::Key(KeyEvent::new(key, mods)))
        .collect()
}
