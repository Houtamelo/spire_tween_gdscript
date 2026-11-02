//! New templates exercised with real Godot nodes, properties and callbacks.
use godot::{classes::*, prelude::*};
use spire_tween::prelude::*;

use super::{misc::MiscTests, util::PinnedTestTask};

#[derive(GodotClass)]
#[class(init, base = Node)]
struct Group4Probe {
    #[var]
    margins: Vector4i,
    #[var]
    number: i64,
    #[var]
    text: GString,
}

#[godot_api]
impl Group4Probe {
    #[func]
    fn receive_number(&mut self, value: i64) { self.number = value; }
}

fn done() -> PinnedTestTask { Box::pin(async {}) }
fn near(a: f64, b: f64) {
    assert!((a - b).abs() < 0.002, "{a} != {b}");
}
fn v2(a: Vector2, b: Vector2) { near(a.distance_to(b) as f64, 0.0); }
fn v3(a: Vector3, b: Vector3) { near(a.distance_to(b) as f64, 0.0); }
macro_rules! manual {
    ($t:expr) => {
        TweenPtr::new(
            $t.with_process_mode(ProcessMode::Manual)
                .with_ease(EaseKind::Basic(Ease::Linear)),
        )
    };
}
fn text_options(rich: bool, mode: ScrambleMode, alphabet: &str) -> TextOptions {
    TextOptions {
        rich_text_enabled: rich,
        scramble_mode: mode,
        scramble_chars: alphabet.into(),
    }
}

impl MiscTests {
    pub(super) fn test_group4_text_buffers(&mut self) -> PinnedTestTask {
        let mut n = Group4Probe::new_alloc();
        self.base_mut().add_child(&n);
        let open = format!("[long_effect url=\"日本]語\"]{}", "[b]".repeat(12));
        let close = format!("{}[/long_effect]", "[/b]".repeat(12));
        let mut t = manual!(
            n.do_property("text", Evaluator::Static(GString::new()), 16.0)
                .begin_from(GString::new())
                .with_text_options(text_options(true, ScrambleMode::None, ""))
        );
        let mut elapsed = 0.0;
        let mut target = GString::new();
        // Reuse one tween through growth, shrinkage, empty input, and nested styles
        // beyond the inline style buffers. The quoted ']' must stay inside its tag.
        for length in [512, 7, 1024, 0, 96] {
            target = GString::from(&format!("{open}{}{close}", "猫".repeat(length)));
            t.set_final_value(target.clone());
            let _ = t.custom_step(0.5, false);
            elapsed += 0.5;
            let count = (length as f64 * elapsed / 16.0).round() as usize;
            let expected = if count == 0 { String::new() } else { format!("{open}{}{close}", "猫".repeat(count)) };
            assert_eq!(n.bind().text, GString::from(&expected));
        }
        t.set_text_options(TextOptions::default());
        let _ = t.custom_step(0.5, false);
        elapsed += 0.5;
        let count = (target.len() as f64 * elapsed / 16.0).round() as usize;
        assert_eq!(n.bind().text, GString::from(&target.chars()[..count]));
        t.set_text_options(text_options(true, ScrambleMode::None, ""));
        let _ = t.custom_step(0.5, false);
        elapsed += 0.5;
        assert_eq!(
            n.bind().text,
            GString::from(&format!("{open}{}{close}", "猫".repeat((96.0 * elapsed / 16.0).round() as usize)))
        );
        t.stop();

        // A style shared by the source and target must remain one balanced run.
        let source = GString::from(&format!("{open}{}{close}", "α".repeat(512)));
        let target = GString::from(&format!("{open}{}{close}", "Z".repeat(512)));
        n.bind_mut().text = source;
        let mut speed = manual!(
            n.do_property("text", Evaluator::Static(target.clone()), 512.0)
                .as_speed_based()
                .with_text_options(text_options(true, ScrambleMode::None, ""))
        );
        let _ = speed.custom_step(0.5, false);
        assert_eq!(n.bind().text, GString::from(&format!("{open}{}{}{close}", "Z".repeat(256), "α".repeat(256))));
        speed.force_complete();
        assert_eq!(n.bind().text, target);

        // Exercise the plain string SmallVec spill as well as the rich cache.
        n.bind_mut().text = GString::from(&"α".repeat(512));
        let plain = manual!(n.do_property("text", Evaluator::Static(GString::from(&"Z".repeat(512))), 1.0));
        let _ = plain.custom_step(0.5, false);
        assert_eq!(n.bind().text, GString::from(&format!("{}{}", "Z".repeat(256), "α".repeat(256))));
        for tag in [
            "lb", "rb", "lrm", "rlm", "lre", "rle", "lro", "rlo", "pdf", "alm", "lri", "rli", "fsi", "pdi", "zwj",
            "zwnj", "wj", "shy",
        ] {
            let t = manual!(
                n.do_property("text", Evaluator::Static(GString::from(&format!("[{tag}]ABC"))), 1.0)
                    .begin_from(GString::new())
                    .with_text_options(text_options(true, ScrambleMode::None, ""))
            );
            let _ = t.custom_step(0.5, false);
            assert_eq!(n.bind().text, GString::from(&format!("[{tag}]A")));
        }
        let mut scramble = manual!(
            n.do_property("text", Evaluator::Static(GString::from(&format!("{open}ABCD{close}"))), 1.0)
                .begin_from(GString::new())
                .with_text_options(text_options(true, ScrambleMode::Custom, &"猫".repeat(100)))
        );
        let _ = scramble.custom_step(0.25, false);
        assert_eq!(n.bind().text, GString::from(&format!("{open}A猫猫猫{close}")));
        scramble.set_text_options(text_options(true, ScrambleMode::Custom, "["));
        let _ = scramble.custom_step(0.25, false);
        assert_eq!(n.bind().text, GString::from(&format!("{open}AB[lb][lb]{close}")));
        scramble.force_complete();
        assert_eq!(n.bind().text, GString::from(&format!("{open}ABCD{close}")));
        n.free();
        done()
    }

    pub(super) fn test_group4_text_plain(&mut self) -> PinnedTestTask {
        let mut n = Label::new_alloc();
        self.base_mut().add_child(&n);
        n.set_text("abcd");
        let mut t = manual!(n.do_text("WXYZ".into(), 1.0).with_text_options(TextOptions::default()));
        n.set_text("αβγδ"); // deferred capture
        let _ = t.custom_step(0.5, false);
        assert_eq!(n.get_text().to_string(), "WXγδ");
        t.force_complete();
        assert_eq!(n.get_text().to_string(), "WXYZ");
        n.set_text("");
        let t = manual!(
            n.do_text("é猫🦀Z".into(), 1.0)
                .with_text_options(TextOptions::default())
        );
        let _ = t.custom_step(0.5, false);
        assert_eq!(n.get_text().to_string(), "é猫");
        n.free();
        done()
    }

    pub(super) fn test_group4_text_bbcode(&mut self) -> PinnedTestTask {
        let mut n = RichTextLabel::new_alloc();
        self.base_mut().add_child(&n);
        n.set_use_bbcode(true);
        let t = manual!(
            n.do_text("[b]A[color=red]猫[/color]CD[/b]".into(), 1.0)
                .with_text_options(text_options(true, ScrambleMode::None, ""))
        );
        let _ = t.custom_step(0.5, false);
        assert_eq!(n.get_text().to_string(), "[b]A[color=red]猫[/color][/b]");
        assert_eq!(n.get_parsed_text().to_string(), "A猫");
        n.set_text("[i]wxyz[/i]");
        let t = manual!(n.do_text("[b]ABCD[/b]".into(), 1.0).with_text_options(text_options(
            true,
            ScrambleMode::None,
            ""
        )));
        let _ = t.custom_step(0.5, false);
        assert_eq!(n.get_text().to_string(), "[b]AB[/b][i]yz[/i]");
        assert_eq!(n.get_parsed_text().to_string(), "AByz");
        n.set_text("");
        let t = manual!(n.do_text("[lb]A[rb]Z".into(), 1.0).with_text_options(text_options(
            true,
            ScrambleMode::None,
            ""
        )));
        let _ = t.custom_step(0.5, false);
        assert_eq!(n.get_parsed_text().to_string(), "[A");
        n.free();
        done()
    }

    pub(super) fn test_group4_text_scramble(&mut self) -> PinnedTestTask {
        let mut n = Label::new_alloc();
        self.base_mut().add_child(&n);
        for (mode, alphabet) in [
            (ScrambleMode::All, "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789"),
            (ScrambleMode::Uppercase, "ABCDEFGHIJKLMNOPQRSTUVWXYZ"),
            (ScrambleMode::Lowercase, "abcdefghijklmnopqrstuvwxyz"),
            (ScrambleMode::Numerals, "0123456789"),
            (ScrambleMode::Custom, "猫🦀"),
        ] {
            n.set_text("");
            let mut t = manual!(
                n.do_text("abcd".into(), 1.0)
                    .with_text_options(text_options(false, mode, alphabet))
            );
            let _ = t.custom_step(0.5, false);
            let text = n.get_text().to_string();
            assert!(text.starts_with("ab"));
            assert_eq!(text.chars().count(), 4);
            assert!(text.chars().skip(2).all(|c| alphabet.contains(c)));
            t.force_complete();
            assert_eq!(n.get_text().to_string(), "abcd");
        }
        let mut t = manual!(n.do_text("ABCD".into(), 1.0).with_text_options(text_options(
            false,
            ScrambleMode::Custom,
            ""
        )));
        let _ = t.custom_step(0.5, false);
        assert_eq!(n.get_text().len(), 4);
        t.force_complete();
        n.free();
        done()
    }

    pub(super) fn test_group4_text_lifecycle(&mut self) -> PinnedTestTask {
        let mut n = Label::new_alloc();
        self.base_mut().add_child(&n);
        n.set_text("abcd");
        let mut t = manual!(n.do_text("WXYZ".into(), 1.0).with_text_options(TextOptions::default()));
        t.set_loops(2, LoopMode::Yoyo);
        let _ = t.custom_step(1.0, false);
        assert_eq!(n.get_text().to_string(), "WXYZ");
        let _ = t.custom_step(0.5, false);
        assert_eq!(n.get_text().to_string(), "WXcd");
        let _ = t.custom_step(0.5, false);
        assert_eq!(n.get_text().to_string(), "abcd");
        t.play();
        let _ = t.custom_step(0.25, false);
        t.stop();
        let stopped = n.get_text();
        t.force_complete();
        assert_eq!(n.get_text(), stopped);
        let t = manual!(n.do_text("end".into(), 1.0).with_text_options(TextOptions::default()));
        n.free();
        assert!(matches!(t.custom_step(0.5, false), TweenStepResult::Stepped(AdvanceTimeResult::ShouldDespawn)));
        done()
    }

    pub(super) fn test_group4_text_relative(&mut self) -> PinnedTestTask {
        let mut n = RichTextLabel::new_alloc();
        self.base_mut().add_child(&n);
        n.set_use_bbcode(true);
        n.set_text("[b]Hi[/b]");
        let mut t = manual!(
            n.do_text("[i]猫Z[/i]".into(), 1.0)
                .as_relative(GString::new())
                .with_text_options(text_options(true, ScrambleMode::Custom, "#"))
        );
        t.set_loops(2, LoopMode::Yoyo);
        let _ = t.custom_step(0.5, false);
        assert!(t.is_relative());
        assert_eq!(n.get_text().to_string(), "[b]Hi[/b][i]猫[/i]");
        let _ = t.custom_step(0.5, false);
        assert_eq!(n.get_parsed_text().to_string(), "Hi猫Z");
        let _ = t.custom_step(0.5, false);
        assert_eq!(n.get_text().to_string(), "[b]Hi[/b][i]猫[/i]");
        let _ = t.custom_step(0.5, false);
        assert_eq!(n.get_text().to_string(), "[b]Hi[/b]");

        // Forced completion also prepares the mode before taking relative samples.
        let mut t = manual!(
            n.do_text("[i]猫Z[/i]".into(), 1.0)
                .with_text_options(text_options(true, ScrambleMode::Custom, "#"))
                .as_relative(GString::new())
        );
        t.force_complete();
        assert_eq!(n.get_text().to_string(), "[b]Hi[/b][i]猫Z[/i]");

        n.set_text("Hi");
        let mut plain = manual!(
            n.do_text("猫Z".into(), 1.0)
                .with_text_options(text_options(false, ScrambleMode::Custom, "#"))
                .as_relative(GString::new())
        );
        let _ = plain.custom_step(0.5, false);
        assert_eq!(n.get_text().to_string(), "Hi猫");
        plain.force_complete();
        assert_eq!(n.get_text().to_string(), "Hi猫Z");
        n.free();
        done()
    }

    pub(super) fn test_group4_text_speed(&mut self) -> PinnedTestTask {
        let mut n = RichTextLabel::new_alloc();
        self.base_mut().add_child(&n);
        n.set_use_bbcode(true);
        n.set_text("[i]a猫[/i]");
        let t = manual!(
            n.do_text("[b]AXY[/b]".into(), 2.0)
                .with_text_options(text_options(true, ScrambleMode::Custom, "#"))
                .as_speed_based()
        );
        let _ = t.custom_step(0.25, false);
        assert!(t.is_speed_based());
        assert_eq!(n.get_text().to_string(), "[i]a猫[/i]");
        let _ = t.custom_step(0.25, false);
        assert_eq!(n.get_text().to_string(), "[b]A[/b][i]猫[/i]");
        let _ = t.custom_step(0.5, false);
        assert_eq!(n.get_text().to_string(), "[b]AX[/b]");
        let TweenStepResult::Stepped(AdvanceTimeResult::Completed { unscaled_excess_time }) =
            t.custom_step(0.75, false)
        else {
            panic!("text should complete with excess time")
        };
        near(unscaled_excess_time, 0.25);
        assert_eq!(n.get_text().to_string(), "[b]AXY[/b]");

        // Tags carry no distance, so formatting-only targets finish immediately.
        n.set_text("[i]A[/i]");
        let t = manual!(
            n.do_text("[b]A[/b]".into(), 2.0)
                .as_speed_based()
                .with_text_options(text_options(true, ScrambleMode::None, ""))
        );
        assert!(matches!(t.custom_step(0.1, false), TweenStepResult::Stepped(AdvanceTimeResult::Completed { .. })));
        assert_eq!(n.get_text().to_string(), "[b]A[/b]");

        n.set_text("[i]AB猫[/i]");
        let mut shrink = manual!(
            n.do_text("[b]A[/b]".into(), 2.0)
                .as_speed_based()
                .with_text_options(text_options(true, ScrambleMode::None, ""))
        );
        let _ = shrink.custom_step(0.5, false);
        assert_eq!(n.get_text().to_string(), "[b]A[/b][i]B[/i]");
        shrink.force_complete();
        assert_eq!(n.get_text().to_string(), "[b]A[/b]");

        n.set_text("[i]ab[/i]");
        let mut yoyo = manual!(
            n.do_text("[b]XY[/b]".into(), 2.0)
                .with_text_options(text_options(true, ScrambleMode::None, ""))
                .as_speed_based()
        );
        yoyo.set_loops(2, LoopMode::Yoyo);
        let _ = yoyo.custom_step(1.0, false);
        assert_eq!(n.get_text().to_string(), "[b]XY[/b]");
        let _ = yoyo.custom_step(0.5, false);
        assert_eq!(n.get_text().to_string(), "[i]a[/i][b]Y[/b]");
        let _ = yoyo.custom_step(0.5, false);
        assert_eq!(n.get_text().to_string(), "[i]ab[/i]");

        n.set_text("");
        let plain = manual!(
            n.do_text("ab".into(), 2.0)
                .as_speed_based()
                .with_text_options(text_options(false, ScrambleMode::Custom, "#"))
        );
        let _ = plain.custom_step(0.5, false);
        assert_eq!(n.get_text().to_string(), "a");
        let _ = plain.custom_step(0.5, false);
        assert_eq!(n.get_text().to_string(), "ab");
        n.free();

        // Image contents remain a single atomic unit without loading the resource.
        let n = Group4Probe::new_alloc();
        self.base_mut().add_child(&n);
        let image = "[img]res://image.png[/img]";
        let t = manual!(
            n.do_property("text", Evaluator::Static(GString::from(image)), 2.0)
                .with_text_options(text_options(true, ScrambleMode::None, ""))
                .as_speed_based()
        );
        assert!(matches!(t.custom_step(0.5, false), TweenStepResult::Stepped(AdvanceTimeResult::Completed { .. })));
        assert_eq!(n.bind().text.to_string(), image);
        n.free();
        done()
    }

    pub(super) fn test_group4_text_options_api(&mut self) -> PinnedTestTask {
        let mut n = Label::new_alloc();
        self.base_mut().add_child(&n);
        n.set_text("abcd");
        let mut t: TweenPtr<LerpPropertyData<GString>> = manual!(n.do_text("X".into(), 1.0));
        assert!(!t.get_text_options().rich_text_enabled);
        let _ = t.custom_step(0.5, false);
        assert_eq!(n.get_text().to_string(), "Xb"); // preserve shrinking half-step rounding
        t.stop();
        n.set_text("abcd");
        let mut t = manual!(n.do_text("[b]WXYZ[/b]".into(), 1.0).with_text_options(text_options(
            true,
            ScrambleMode::None,
            ""
        )));
        let _ = t.custom_step(0.25, false);
        assert_eq!(n.get_text().to_string(), "[b]W[/b]bcd");
        t.set_final_value("[i]1234[/i]".into());
        let _ = t.custom_step(0.25, false);
        assert_eq!(n.get_text().to_string(), "[i]12[/i]cd");
        t.set_dynamic_target(Callable::from_fn("text_target", |_| "[u]5678[/u]".to_variant()));
        let _ = t.custom_step(0.25, false);
        assert_eq!(n.get_text().to_string(), "[u]567[/u]d");
        t.force_complete();
        assert_eq!(n.get_text().to_string(), "[u]5678[/u]");

        n.set_text("");
        let mut t = manual!(n.do_text("abcd".into(), 1.0).with_text_options(text_options(
            false,
            ScrambleMode::Custom,
            "#"
        )));
        let _ = t.custom_step(0.25, false);
        assert_eq!(n.get_text().to_string(), "a###");
        t.set_text_options(TextOptions::default());
        let _ = t.custom_step(0.25, false);
        assert_eq!(n.get_text().to_string(), "ab");

        let mut rich = manual!(
            n.do_text("[b]X[/b]".into(), 1.0)
                .begin_from("[i]abcd[/i]".into())
                .with_text_options(text_options(true, ScrambleMode::None, ""))
        );
        let _ = rich.custom_step(0.5, false);
        assert_eq!(n.get_text().to_string(), "[b]X[/b][i]b[/i]");
        rich.force_complete();
        assert_eq!(n.get_text().to_string(), "[b]X[/b]");
        n.free();
        done()
    }

    pub(super) fn test_group4_path_linear(&mut self) -> PinnedTestTask {
        let mut n = Node2D::new_alloc();
        self.base_mut().add_child(&n);
        let t =
            manual!(n.do_path(vec![Vector2::new(10.0, 0.0), Vector2::new(10.0, 30.0)], 1.0, PathOptions::default()));
        let _ = t.custom_step(0.5, false);
        v2(n.get_position(), Vector2::new(10.0, 10.0));
        n.set_position(Vector2::ZERO);
        let mut t = manual!(n.do_path(vec![Vector2::new(20.0, 0.0)], 1.0, PathOptions::default()));
        n.set_position(Vector2::new(10.0, 0.0));
        let _ = t.custom_step(0.5, false);
        v2(n.get_position(), Vector2::new(15.0, 0.0));
        t.force_complete();
        v2(n.get_position(), Vector2::new(20.0, 0.0));
        n.free();
        done()
    }

    pub(super) fn test_group4_path_curves(&mut self) -> PinnedTestTask {
        let mut n = Node2D::new_alloc();
        self.base_mut().add_child(&n);
        let o = PathOptions {
            path_type: PathType::CubicBezier,
            resolution: 100,
            ..Default::default()
        };
        let t = manual!(n.do_path(
            vec![Vector2::new(10.0, 0.0), Vector2::new(0.0, 10.0), Vector2::new(10.0, 10.0)],
            1.0,
            o
        ));
        let _ = t.custom_step(0.5, false);
        v2(n.get_position(), Vector2::new(5.0, 7.5));
        n.set_position(Vector2::ZERO);
        let o = PathOptions {
            path_type: PathType::CatmullRom,
            resolution: 100,
            ..Default::default()
        };
        let t = manual!(n.do_path(vec![Vector2::new(10.0, 10.0), Vector2::new(20.0, 0.0)], 1.0, o));
        let _ = t.custom_step(0.5, false);
        v2(n.get_position(), Vector2::new(10.0, 10.0));
        n.free();
        done()
    }

    pub(super) fn test_group4_path_spaces(&mut self) -> PinnedTestTask {
        let mut parent = Node2D::new_alloc();
        self.base_mut().add_child(&parent);
        parent.set_position(Vector2::new(10.0, 20.0));
        parent.set_rotation(0.5);
        parent.set_scale(Vector2::new(2.0, 3.0));
        let n = Node2D::new_alloc();
        parent.add_child(&n);
        let mut t = manual!(n.do_path(vec![Vector2::new(5.0, 8.0)], 1.0, PathOptions::default()));
        t.force_complete();
        v2(n.get_position(), Vector2::new(5.0, 8.0));
        v2(n.get_global_position(), parent.to_global(Vector2::new(5.0, 8.0)));
        let mut t = manual!(n.do_global_path(vec![Vector2::new(20.0, 40.0)], 1.0, PathOptions::default()));
        t.force_complete();
        v2(n.get_global_position(), Vector2::new(20.0, 40.0));
        parent.free();
        done()
    }

    pub(super) fn test_group4_path_3d(&mut self) -> PinnedTestTask {
        let mut parent = Node3D::new_alloc();
        self.base_mut().add_child(&parent);
        parent.set_position(Vector3::new(10.0, 20.0, 30.0));
        parent.set_rotation(Vector3::new(0.2, 0.4, 0.3));
        let mut n = Node3D::new_alloc();
        parent.add_child(&n);
        n.set_scale(Vector3::new(2.0, 3.0, 4.0));
        let o = PathOptions {
            path_type: PathType::CubicBezier,
            resolution: 100,
            ..Default::default()
        };
        let mut t = manual!(n.do_path(
            vec![
                Vector3::new(10.0, 0.0, 0.0),
                Vector3::new(0.0, 10.0, 0.0),
                Vector3::new(10.0, 10.0, 0.0)
            ],
            1.0,
            o
        ));
        let _ = t.custom_step(0.5, false);
        v3(n.get_position(), Vector3::new(5.0, 7.5, 0.0));
        t.force_complete();
        v3(n.get_position(), Vector3::new(10.0, 0.0, 0.0));
        let mut t = manual!(n.do_global_path(vec![Vector3::new(1.0, 2.0, 3.0)], 1.0, PathOptions::default()));
        t.force_complete();
        v3(n.get_global_position(), Vector3::new(1.0, 2.0, 3.0));
        v3(n.get_scale(), Vector3::new(2.0, 3.0, 4.0));
        parent.free();
        done()
    }

    pub(super) fn test_group4_path_control(&mut self) -> PinnedTestTask {
        let mut n = Control::new_alloc();
        self.base_mut().add_child(&n);
        let mut t = manual!(n.do_path(vec![Vector2::new(0.0, 10.0)], 1.0, PathOptions {
            orient: true,
            ..Default::default()
        }));
        let _ = t.custom_step(0.5, false);
        v2(n.get_position(), Vector2::new(0.0, 5.0));
        near(n.get_rotation() as f64, std::f64::consts::FRAC_PI_2);
        t.force_complete();
        v2(n.get_position(), Vector2::new(0.0, 10.0));
        n.free();
        done()
    }

    pub(super) fn test_group4_path_closed_locks(&mut self) -> PinnedTestTask {
        let mut n = Node2D::new_alloc();
        self.base_mut().add_child(&n);
        let t = manual!(n.do_path(vec![Vector2::new(10.0, 0.0)], 1.0, PathOptions {
            closed: true,
            ..Default::default()
        }));
        let _ = t.custom_step(0.5, false);
        v2(n.get_position(), Vector2::new(10.0, 0.0));
        let _ = t.custom_step(0.5, false);
        v2(n.get_position(), Vector2::ZERO);
        let t = manual!(n.do_path(vec![Vector2::new(10.0, 30.0), Vector2::new(30.0, 30.0)], 1.0, PathOptions {
            lock_position: 2,
            ..Default::default()
        }));
        let _ = t.custom_step(0.5, false);
        v2(n.get_position(), Vector2::new(15.0, 0.0));
        n.free();
        done()
    }

    pub(super) fn test_group4_path_orientation(&mut self) -> PinnedTestTask {
        let mut n = Node3D::new_alloc();
        self.base_mut().add_child(&n);
        n.set_scale(Vector3::new(2.0, 3.0, 4.0));
        let mut t = manual!(n.do_path(vec![Vector3::new(10.0, 0.0, 0.0)], 1.0, PathOptions {
            orient: true,
            ..Default::default()
        }));
        let _ = t.custom_step(0.5, false);
        v3(-n.get_global_basis().orthonormalized().col_c(), Vector3::RIGHT);
        v3(n.get_scale(), Vector3::new(2.0, 3.0, 4.0));
        t.force_complete();
        v3(-n.get_global_basis().orthonormalized().col_c(), Vector3::RIGHT);
        n.set_rotation(Vector3::ZERO);
        let mut t = manual!(n.do_path(vec![Vector3::new(20.0, 0.0, 0.0)], 1.0, PathOptions {
            orient: true,
            use_look_at_position: true,
            look_at_position: Vector3::new(15.0, 0.0, -10.0),
            ..Default::default()
        }));
        let _ = t.custom_step(0.5, false);
        v3(-n.get_global_basis().orthonormalized().col_c(), Vector3::FORWARD);
        n.free();
        done()
    }

    pub(super) fn test_group4_path_lifecycle(&mut self) -> PinnedTestTask {
        let mut n = Node2D::new_alloc();
        self.base_mut().add_child(&n);
        for mode in [LoopMode::Restart, LoopMode::Yoyo, LoopMode::Incremental] {
            n.set_position(Vector2::ZERO);
            let mut t = manual!(n.do_path(vec![Vector2::new(10.0, 0.0)], 1.0, PathOptions::default()));
            t.set_loops(2, mode);
            let _ = t.custom_step(1.0, false);
            let _ = t.custom_step(0.5, false);
            v2(n.get_position(), Vector2::new(if mode == LoopMode::Incremental { 15.0 } else { 5.0 }, 0.0));
            t.stop();
            let position = n.get_position();
            t.force_complete();
            v2(n.get_position(), position);
        }
        n.set_position(Vector2::ZERO);
        let mut child = n
            .do_path(vec![Vector2::new(10.0, 0.0)], 1.0, PathOptions::default())
            .with_process_mode(ProcessMode::Manual);
        child.set_speed_scale(2.0);
        let mut seq = SpireTween::<Sequence>::new();
        seq.append(child);
        seq.append(
            n.do_position(Vector2::new(20.0, 0.0), 1.0)
                .with_ease(EaseKind::Basic(Ease::Linear)),
        );
        let t = TweenPtr::new(seq.with_process_mode(ProcessMode::Manual));
        let _ = t.custom_step(0.75, false);
        v2(n.get_position(), Vector2::new(12.5, 0.0));
        n.free();
        done()
    }

    pub(super) fn test_group4_path_invalid(&mut self) -> PinnedTestTask {
        let mut n = Node2D::new_alloc();
        self.base_mut().add_child(&n);
        n.set_position(Vector2::new(2.0, 3.0));
        for points in [vec![], vec![Vector2::new(2.0, 3.0); 3]] {
            let mut t = manual!(n.do_path(points, 1.0, PathOptions::default()));
            t.force_complete();
            v2(n.get_position(), Vector2::new(2.0, 3.0));
        }
        let mut t = manual!(n.do_path(vec![Vector2::new(f32::NAN, 1.0)], f64::NAN, PathOptions {
            path_type: PathType::CubicBezier,
            resolution: 0,
            look_ahead: f64::NAN,
            up: Vector3::ZERO,
            forward_direction: Vector3::ZERO,
            lock_position: 99,
            ..Default::default()
        }));
        near(t.get_duration(), 1.0);
        t.force_complete();
        v2(n.get_position(), Vector2::new(2.0, 3.0));
        n.free();
        done()
    }

    pub(super) fn test_group4_path_cancellation(&mut self) -> PinnedTestTask {
        let n = Node3D::new_alloc();
        self.base_mut().add_child(&n);
        let target = Node3D::new_alloc();
        self.base_mut().add_child(&target);
        let t = manual!(n.do_path(vec![Vector3::ONE], 1.0, PathOptions {
            orient: true,
            look_at_target: Some(target.clone().upcast()),
            ..Default::default()
        }));
        target.free();
        assert!(matches!(t.custom_step(0.5, false), TweenStepResult::NoLongerValid));
        let t = manual!(n.do_path(vec![Vector3::ONE], 1.0, PathOptions::default()));
        n.free();
        assert!(matches!(t.custom_step(0.5, false), TweenStepResult::Stepped(AdvanceTimeResult::ShouldDespawn)));
        done()
    }

    pub(super) fn test_group4_margins(&mut self) -> PinnedTestTask {
        let mut n = MarginContainer::new_alloc();
        self.base_mut().add_child(&n);
        for (name, v) in [("margin_left", 0), ("margin_right", 2), ("margin_top", 4), ("margin_bottom", 6)] {
            n.add_theme_constant_override(name, v);
        }
        let mut t = manual!(n.do_margins(Vector4i::new(4, 6, 8, 10), 1.0));
        let _ = t.custom_step(0.5, false);
        assert_eq!(n.get_theme_constant("margin_left"), 2);
        assert_eq!(n.get_theme_constant("margin_bottom"), 8);
        t.force_complete();
        assert_eq!(n.get_theme_constant("margin_top"), 8);
        n.free();
        let n = Group4Probe::new_alloc();
        self.base_mut().add_child(&n);
        let mut t = manual!(do_property_margins(n.clone().upcast(), "margins".into(), Vector4i::new(-2, 4, 6, 8), 1.0));
        t.force_complete();
        assert_eq!(n.bind().margins, Vector4i::new(-2, 4, 6, 8));
        let mut invalid = manual!(do_property_margins(n.clone().upcast(), "number".into(), Vector4i::ONE, 1.0));
        invalid.force_complete();
        assert_eq!(n.bind().number, 0);
        n.free();
        done()
    }

    pub(super) fn test_group4_integer_api(&mut self) -> PinnedTestTask {
        let mut n = Group4Probe::new_alloc();
        self.base_mut().add_child(&n);
        n.bind_mut().number = -5_000_000_000;
        let mut t = manual!(n.do_property("number", Evaluator::Static(5_000_000_000_i64), 1.0));
        let _ = t.custom_step(0.5, false);
        assert_eq!(n.bind().number, 0);
        t.force_complete();
        assert_eq!(n.bind().number, 5_000_000_000);
        let mut t = manual!(n.do_method("receive_number", 6_i64, -6_i64, 1.0));
        let _ = t.custom_step(0.75, false);
        assert_eq!(n.bind().number, -3);
        t.force_complete();
        assert_eq!(n.bind().number, -6);
        let invalid = manual!(SpireTween::<LerpMethodData<i64>>::new(Callable::invalid(), -6, 6, 1.0));
        assert!(matches!(invalid.custom_step(0.5, false), TweenStepResult::Stepped(AdvanceTimeResult::ShouldDespawn)));
        assert_eq!(invalid.get_state(), TweenState::Stopped);
        let t = manual!(n.do_method("receive_number", -6_i64, 6_i64, 1.0));
        n.free();
        assert!(matches!(t.custom_step(0.5, false), TweenStepResult::Stepped(AdvanceTimeResult::ShouldDespawn)));
        done()
    }

    pub(super) fn test_group4_text_markup_edges(&mut self) -> PinnedTestTask {
        let mut n = Group4Probe::new_alloc();
        self.base_mut().add_child(&n);
        for (source, expected) in [("[br]ABC", "[br]"), ("[lrm]ABC", "[lrm]A")] {
            n.bind_mut().text = GString::new();
            let t = manual!(
                n.do_property("text", Evaluator::Static(source.into()), 1.0)
                    .with_text_options(text_options(true, ScrambleMode::None, ""))
            );
            let _ = t.custom_step(0.5, false);
            assert_eq!(n.bind().text.to_string(), expected);
        }
        n.bind_mut().text = GString::new();
        let mut t = manual!(
            n.do_property("text", Evaluator::Static("[img]res://image.png[/img]XY".into()), 1.0)
                .with_text_options(text_options(true, ScrambleMode::None, ""))
        );
        let _ = t.custom_step(0.5, false);
        assert_eq!(n.bind().text.to_string(), "[img]res://image.png[/img]X");
        t.force_complete();
        assert_eq!(n.bind().text.to_string(), "[img]res://image.png[/img]XY");
        n.bind_mut().text = GString::new();
        let t = manual!(
            n.do_property("text", Evaluator::Static("[url=\"a]b\"]ABCD[/url]".into()), 1.0)
                .with_text_options(text_options(true, ScrambleMode::None, ""))
        );
        let _ = t.custom_step(0.5, false);
        assert_eq!(n.bind().text.to_string(), "[url=\"a]b\"]AB[/url]");
        n.free();
        done()
    }

    pub(super) fn test_group4_path_curve_edges(&mut self) -> PinnedTestTask {
        let mut n = Node3D::new_alloc();
        self.base_mut().add_child(&n);
        let mut t =
            manual!(n.do_path(vec![Vector3::new(10.0, 10.0, 0.0), Vector3::new(20.0, 0.0, 0.0)], 1.0, PathOptions {
                path_type: PathType::CatmullRom,
                resolution: 100,
                ..Default::default()
            }));
        let _ = t.custom_step(0.5, false);
        v3(n.get_position(), Vector3::new(10.0, 10.0, 0.0));
        t.force_complete();
        for kind in [PathType::CatmullRom, PathType::CubicBezier] {
            n.set_position(Vector3::ZERO);
            let points = if kind == PathType::CubicBezier {
                vec![
                    Vector3::new(10.0, 0.0, 0.0),
                    Vector3::new(3.0, 0.0, 0.0),
                    Vector3::new(7.0, 0.0, 0.0),
                ]
            } else {
                vec![Vector3::new(10.0, 0.0, 0.0)]
            };
            let mut t = manual!(n.do_path(points, 1.0, PathOptions {
                closed: true,
                path_type: kind,
                ..Default::default()
            }));
            let _ = t.custom_step(0.5, false);
            v3(n.get_position(), Vector3::new(10.0, 0.0, 0.0));
            t.force_complete();
            v3(n.get_position(), Vector3::ZERO);
        }
        let mut t = manual!(n.do_path(
            vec![Vector3::ZERO, Vector3::new(0.0, 10.0, 0.0), Vector3::new(10.0, 10.0, 0.0)],
            1.0,
            PathOptions {
                path_type: PathType::CubicBezier,
                resolution: 1,
                ..Default::default()
            }
        ));
        let _ = t.custom_step(0.25, false);
        assert!(n.get_position().length() > 1.0);
        t.force_complete();
        v3(n.get_position(), Vector3::ZERO);
        n.free();
        done()
    }

    pub(super) fn test_group4_path_modes(&mut self) -> PinnedTestTask {
        let mut n = Node3D::new_alloc();
        self.base_mut().add_child(&n);
        for (mode, axis) in [
            (PathMode::Ignore, Vector3::ZERO),
            (PathMode::SideScroller2D, Vector3::new(0.0, 0.0, std::f32::consts::FRAC_PI_2)),
            (PathMode::TopDown2D, Vector3::new(0.0, -std::f32::consts::FRAC_PI_2, 0.0)),
        ] {
            n.set_position(Vector3::ZERO);
            n.set_rotation(Vector3::ZERO);
            let point = if mode == PathMode::SideScroller2D {
                Vector3::new(0.0, 10.0, 0.0)
            } else {
                Vector3::new(10.0, 0.0, 0.0)
            };
            let mut t = manual!(n.do_path(vec![point], 1.0, PathOptions {
                orient: true,
                mode,
                ..Default::default()
            }));
            t.force_complete();
            v3(n.get_rotation(), axis);
        }
        n.set_position(Vector3::ZERO);
        n.set_rotation(Vector3::ZERO);
        let mut t = manual!(n.do_path(vec![Vector3::new(10.0, 0.0, 0.0)], 1.0, PathOptions {
            orient: true,
            lock_rotation: 7,
            ..Default::default()
        }));
        t.force_complete();
        v3(n.get_rotation(), Vector3::ZERO);
        n.set_position(Vector3::ZERO);
        let mut t = manual!(n.do_path(vec![Vector3::new(10.0, 0.0, 0.0)], 1.0, PathOptions {
            orient: true,
            forward_direction: Vector3::RIGHT,
            ..Default::default()
        }));
        t.force_complete();
        v3(n.get_basis().col_a().normalized(), Vector3::RIGHT);
        n.free();
        done()
    }

    pub(super) fn test_group4_path_targets(&mut self) -> PinnedTestTask {
        let n = Node2D::new_alloc();
        self.base_mut().add_child(&n);
        let mut target = Node2D::new_alloc();
        self.base_mut().add_child(&target);
        target.set_position(Vector2::new(5.0, 10.0));
        let t = manual!(n.do_path(vec![Vector2::new(10.0, 0.0)], 1.0, PathOptions {
            orient: true,
            look_at_target: Some(target.clone().upcast()),
            ..Default::default()
        }));
        let _ = t.custom_step(0.5, false);
        near(n.get_global_rotation() as f64, std::f64::consts::FRAC_PI_2);
        target.set_position(Vector2::new(7.5, -10.0));
        let _ = t.custom_step(0.25, false);
        near(n.get_global_rotation() as f64, -std::f64::consts::FRAC_PI_2);
        let mut ignored = manual!(n.do_path(vec![Vector2::new(20.0, 0.0)], 1.0, PathOptions {
            orient: false,
            look_at_target: Some(target.clone().upcast()),
            ..Default::default()
        }));
        target.free();
        ignored.force_complete();
        v2(n.get_position(), Vector2::new(20.0, 0.0));
        n.free();
        done()
    }

    pub(super) fn test_group4_margin_accessors(&mut self) -> PinnedTestTask {
        let value = RcPtr::new(Vector4i::ZERO);
        let reader = value.clone();
        let mut writer = value.clone();
        let mut t = manual!(do_get_set_margins(move || *reader, move |v| *writer = v, Vector4i::new(4, 6, 8, 10), 1.0));
        let mut editor = value.clone();
        *editor = Vector4i::new(2, 4, 6, 8);
        let _ = t.custom_step(0.5, false);
        assert_eq!(*value, Vector4i::new(3, 5, 7, 9));
        t.stop();
        t.force_complete();
        assert_eq!(*value, Vector4i::new(3, 5, 7, 9));
        done()
    }

    pub(super) fn test_group4_path_zero_scale_top_level(&mut self) -> PinnedTestTask {
        let mut parent = Node2D::new_alloc();
        self.base_mut().add_child(&parent);
        parent.set_position(Vector2::new(30.0, 40.0));
        parent.set_rotation(0.5);
        let mut n = Node2D::new_alloc();
        parent.add_child(&n);
        n.set_as_top_level(true);
        n.set_scale(Vector2::ZERO);
        let mut t = manual!(n.do_path(vec![Vector2::new(0.0, 10.0)], 1.0, PathOptions {
            orient: true,
            ..Default::default()
        }));
        t.force_complete();
        v2(n.get_global_position(), Vector2::new(0.0, 10.0));
        let mut control = Control::new_alloc();
        parent.add_child(&control);
        control.set_scale(Vector2::ZERO);
        let mut t = manual!(control.do_path(vec![Vector2::new(10.0, 0.0)], 1.0, PathOptions {
            orient: true,
            ..Default::default()
        }));
        t.force_complete();
        v2(control.get_position(), Vector2::new(10.0, 0.0));
        parent.free();
        let mut parent = Node3D::new_alloc();
        self.base_mut().add_child(&parent);
        parent.set_position(Vector3::new(30.0, 40.0, 50.0));
        let mut n = Node3D::new_alloc();
        parent.add_child(&n);
        n.set_as_top_level(true);
        n.set_scale(Vector3::ZERO);
        let mut t = manual!(n.do_path(vec![Vector3::RIGHT * 10.0], 1.0, PathOptions {
            orient: true,
            ..Default::default()
        }));
        t.force_complete();
        v3(n.get_global_position(), Vector3::RIGHT * 10.0);
        parent.free();
        done()
    }
}
