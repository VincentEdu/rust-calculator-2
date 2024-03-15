mod calc;

use std::cell::RefCell;
use std::rc::Rc;

use calc::Calculator;
use calc::Feature;

use calc::EXP_UNIT_NAME_ADD;
use calc::EXP_UNIT_NAME_DIV;
use calc::EXP_UNIT_NAME_INV;
use calc::EXP_UNIT_NAME_MUL;
use calc::EXP_UNIT_NAME_SQR;
use calc::EXP_UNIT_NAME_SQRT;
use calc::EXP_UNIT_NAME_SUB;
use druid::{
    theme, AppLauncher, Color, Data, Lens, LocalizedString, RenderContext, Widget, WidgetExt,
    WindowDesc,
};

use druid::widget::{CrossAxisAlignment, Flex, Label, Painter};

#[derive(Clone, Data, Lens)]
struct AppData {
    history: String,
    value: String,
    caculator: Rc<RefCell<Calculator>>,
}


impl AppData {
    fn on_exp_key(&mut self, key: String) {
        let mut caculator = self.caculator.borrow_mut();
        let state = caculator.perform_exp_input(key);
        match state {
            Ok(t) => {
                match t {
                    Some(res) => {
                        self.history = caculator.build_history();
                        self.value = res;
                    }
                    None => {
                        self.history = caculator.build_history();
                    }
                }
            },
            Err(s) => {
                self.value = s;
                self.history = caculator.build_history();

                let _ = caculator.reset();
            }
        };
    }

    fn on_feature_key(&mut self, feature: &Feature) {
        let mut caculator = self.caculator.borrow_mut();
        let state = caculator.perform_feature(feature);
        match state {
            Ok(t) => {
                match t {
                    Some(res) => {
                        self.history = caculator.build_history();
                        self.value = res;
                    }
                    None => {
                        self.history = caculator.build_history();
                    }
                }
            },
            Err(s) => {
                self.value = s.to_string();
                self.history = caculator.build_history();

                let _ = caculator.reset();
            }
        };
    }
}

fn op_button_label_id(label: String, id: String) -> impl Widget<AppData> {
    let painter = Painter::new(|ctx, _, env| {
        let bounds = ctx.size().to_rect();

        ctx.fill(bounds, &env.get(theme::PRIMARY_DARK));

        if ctx.is_hot() {
            ctx.stroke(bounds.inset(-0.5), &Color::WHITE, 1.0);
        }

        if ctx.is_active() {
            ctx.fill(bounds, &env.get(theme::PRIMARY_LIGHT));
        }
    });

    Label::new(label)
        .with_text_size(24.)
        .center()
        .background(painter)
        .expand()
        .on_click(move |_ctx, data: &mut AppData, _env| data.on_exp_key(id.clone()))
}

fn op_button_label(label: String) -> impl Widget<AppData> {
    op_button_label_id(label.clone(), label)
}

fn op_feature(feature: Feature) -> impl Widget<AppData> {
    let painter = Painter::new(|ctx, _, env| {
        let bounds = ctx.size().to_rect();

        ctx.fill(bounds, &env.get(theme::PRIMARY_DARK));

        if ctx.is_hot() {
            ctx.stroke(bounds.inset(-0.5), &Color::WHITE, 1.0);
        }

        if ctx.is_active() {
            ctx.fill(bounds, &env.get(theme::PRIMARY_LIGHT));
        }
    });

    let label = match feature {
        Feature::C => "C",
        Feature::CE => "CE",
        Feature::MS => "MS",
        Feature::MR => "MR",
        Feature::Eval => "=",
        Feature::DEL => "⌫",
    };

    let label_str = label.to_string();

    Label::new(label_str)
        .with_text_size(24.)
        .center()
        .background(painter)
        .expand()
        .on_click(move |_ctx, data: &mut AppData, _env| data.on_feature_key(&feature))
}

fn digit_button(digit: char) -> impl Widget<AppData> {
    let painter = Painter::new(|ctx, _, env| {
        let bounds = ctx.size().to_rect();

        ctx.fill(bounds, &env.get(theme::BACKGROUND_LIGHT));

        if ctx.is_hot() {
            ctx.stroke(bounds.inset(-0.5), &Color::WHITE, 1.0);
        }

        if ctx.is_active() {
            ctx.fill(bounds, &Color::rgb8(0x71, 0x71, 0x71));
        }
    });

    Label::new(digit.to_string())
        .with_text_size(24.)
        .center()
        .background(painter)
        .expand()
        .on_click(move |_ctx, data: &mut AppData, _env| data.on_exp_key(digit.to_string()))
}

fn flex_row<T: Data>(
    w1: impl Widget<T> + 'static,
    w2: impl Widget<T> + 'static,
    w3: impl Widget<T> + 'static,
    w4: impl Widget<T> + 'static,
    w5: impl Widget<T> + 'static,
) -> impl Widget<T> {
    Flex::row()
        .with_flex_child(w1, 1.0)
        .with_spacer(1.0)
        .with_flex_child(w2, 1.0)
        .with_spacer(1.0)
        .with_flex_child(w3, 1.0)
        .with_spacer(1.0)
        .with_flex_child(w4, 1.0)
        .with_spacer(1.0)
        .with_flex_child(w5, 1.0)
}

fn build_calc() -> impl Widget<AppData> {
    let lb_history = Label::new(|data: &String, _env: &_| data.clone())
        .with_text_size(14.0)
        .lens(AppData::history)
        .padding(5.0);
    let lb_result = Label::new(|data: &String, _env: &_| data.clone())
        .with_text_size(20.0)
        .lens(AppData::value)
        .padding(5.0);
    Flex::column()
        .with_flex_spacer(0.2)
        .with_child(lb_history)
        .with_flex_spacer(0.2)
        .with_child(lb_result)
        .with_flex_spacer(0.2)
        .cross_axis_alignment(CrossAxisAlignment::End)
        .with_flex_child(
            flex_row(
                op_feature(Feature::CE),
                op_feature(Feature::C),
                op_feature(Feature::MS),
                op_feature(Feature::MR),
                op_feature(Feature::DEL),
            ),
            1.0,
        )
        .with_spacer(1.0)
        .with_flex_child(
            flex_row(
                op_button_label("(".to_string()),
                op_button_label(")".to_string()),
                op_button_label_id("⅟x".to_string(), EXP_UNIT_NAME_INV.to_string()),
                op_button_label( "π".to_string()),
                op_button_label(EXP_UNIT_NAME_DIV.to_string()), //
            ),
            1.0,
        )
        .with_spacer(1.0)        
        .with_flex_child(
            flex_row(
                digit_button('7'),
                digit_button('8'),
                digit_button('9'),
                op_button_label("tan".to_string()),
                op_button_label(EXP_UNIT_NAME_MUL.to_string()),
            ),
            1.0,
        )
        .with_spacer(1.0)
        .with_flex_child(
            flex_row(
                digit_button('4'),
                digit_button('5'),
                digit_button('6'),
                op_button_label("cos".to_string()),
                op_button_label(EXP_UNIT_NAME_SUB.to_string()),
            ),
            1.0,
        )
        .with_spacer(1.0)
        .with_flex_child(
            flex_row(
                digit_button('1'),
                digit_button('2'),
                digit_button('3'),
                op_button_label("sin".to_string()),
                op_button_label(EXP_UNIT_NAME_ADD.to_string()),
            ),
            1.0,
        )
        .with_spacer(1.0)
        .with_flex_child(
            flex_row(
                digit_button('0'),
                op_button_label(".".to_string()),
                op_button_label(EXP_UNIT_NAME_SQRT.to_string()),
                op_button_label_id("x²".to_string(), EXP_UNIT_NAME_SQR.to_string()),
                op_feature(Feature::Eval),
            ),
            1.0,
        )
}

pub fn main() {
    let window = WindowDesc::new(build_calc())
        .window_size((403., 400.))
        .resizable(false)
        .title(
            LocalizedString::new("calc-demo-window-title").with_placeholder("Simple Calculator"),
        );

    let app_data: AppData = AppData {
        history: String::new(),
        value: "0".to_string(),
        caculator: Rc::new(RefCell::new(Calculator::new()))
    };

    app_data.caculator.borrow_mut().add_constant("π".to_string(), "3.14159265358979323846".to_string());

    AppLauncher::with_window(window)
        .log_to_console()
        .launch(app_data)
        .expect("launch failed");
}