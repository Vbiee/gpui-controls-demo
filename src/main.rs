use gpui_kit::component::{
    ActiveTheme, Disableable, IndexPath, Theme, ThemeMode,
    button::{Button, ButtonVariants},
    checkbox::Checkbox,
    h_flex,
    input::{Input, InputEvent, InputState},
    progress::Progress,
    radio::{Radio, RadioGroup},
    select::{SearchableVec, Select, SelectState},
    slider::{Slider, SliderEvent, SliderState},
    switch::Switch,
    tab::{Tab, TabBar},
    v_flex,
};
use gpui_kit::*;

struct ControlsDemo {
    click_count: usize,
    name: Entity<InputState>,
    password: Entity<InputState>,
    agreed: bool,
    newsletter: bool,
    dark_mode: bool,
    plan: Option<usize>,
    volume: Entity<SliderState>,
    volume_value: f32,
    language: Entity<SelectState<SearchableVec<&'static str>>>,
    tab: usize,
    status: SharedString,
    _subscriptions: Vec<Subscription>,
}

impl ControlsDemo {
    fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let name = cx.new(|cx| InputState::new(window, cx).placeholder("Your name"));
        let password = cx.new(|cx| {
            InputState::new(window, cx)
                .placeholder("Password")
                .masked(true)
        });
        let volume = cx.new(|_| {
            SliderState::new()
                .min(0.)
                .max(100.)
                .step(1.)
                .default_value(40.)
        });
        let language = cx.new(|cx| {
            SelectState::new(
                SearchableVec::new(vec!["Rust", "Go", "TypeScript", "Python", "C++"]),
                Some(IndexPath::new(0)),
                window,
                cx,
            )
        });

        let _subscriptions = vec![
            cx.subscribe(&name, |this, input, event: &InputEvent, cx| {
                if let InputEvent::Change = event {
                    this.status = format!("Name changed: {}", input.read(cx).value()).into();
                    cx.notify();
                }
            }),
            cx.subscribe(&volume, |this, _, event: &SliderEvent, cx| {
                let SliderEvent::Change(value) = event else {
                    return;
                };
                this.volume_value = value.start();
                cx.notify();
            }),
        ];

        Self {
            click_count: 0,
            name,
            password,
            agreed: false,
            newsletter: true,
            dark_mode: false,
            plan: Some(0),
            volume,
            volume_value: 40.,
            language,
            tab: 0,
            status: "Ready".into(),
            _subscriptions,
        }
    }

    fn section(
        title: &'static str,
        cx: &App,
        content: impl IntoElement,
    ) -> impl IntoElement {
        v_flex()
            .gap_3()
            .p_4()
            .border_1()
            .border_color(cx.theme().border)
            .rounded_lg()
            .child(
                div()
                    .text_sm()
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_color(cx.theme().muted_foreground)
                    .child(title),
            )
            .child(content)
    }
}

impl Render for ControlsDemo {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let buttons = h_flex()
            .gap_2()
            .flex_wrap()
            .child(
                Button::new("primary")
                    .primary()
                    .label(format!("Clicked {} times", self.click_count))
                    .on_click(cx.listener(|this, _, _, cx| {
                        this.click_count += 1;
                        cx.notify();
                    })),
            )
            .child(Button::new("secondary").secondary().label("Secondary"))
            .child(Button::new("outline").outline().label("Outline"))
            .child(Button::new("ghost").ghost().label("Ghost"))
            .child(Button::new("danger").danger().label("Danger"))
            .child(Button::new("link").link().label("Link"))
            .child(Button::new("loading").label("Loading").loading(true))
            .child(Button::new("disabled").label("Disabled").disabled(true));

        let inputs = v_flex()
            .gap_2()
            .child(Input::new(&self.name).w(px(320.)))
            .child(Input::new(&self.password).w(px(320.)));

        let toggles = v_flex()
            .gap_3()
            .child(
                Checkbox::new("agree")
                    .label("I agree to the terms")
                    .checked(self.agreed)
                    .on_change(cx.listener(|this, checked, _, cx| {
                        this.agreed = *checked;
                        cx.notify();
                    })),
            )
            .child(
                Checkbox::new("locked")
                    .label("Disabled checkbox")
                    .checked(true)
                    .disabled(true),
            )
            .child(
                h_flex()
                    .gap_2()
                    .child(
                        Switch::new("newsletter")
                            .checked(self.newsletter)
                            .on_change(cx.listener(|this, checked, _, cx| {
                                this.newsletter = *checked;
                                cx.notify();
                            })),
                    )
                    .child("Subscribe to newsletter"),
            )
            .child(
                h_flex()
                    .gap_2()
                    .child(
                        Switch::new("dark-mode")
                            .checked(self.dark_mode)
                            .on_change(cx.listener(|this, checked, window, cx| {
                                this.dark_mode = *checked;
                                let mode = if *checked {
                                    ThemeMode::Dark
                                } else {
                                    ThemeMode::Light
                                };
                                Theme::change(mode, Some(window), cx);
                                cx.notify();
                            })),
                    )
                    .child("Dark mode"),
            );

        let radios = RadioGroup::horizontal("plan")
            .selected_index(self.plan)
            .child(Radio::new("free").label("Free"))
            .child(Radio::new("pro").label("Pro"))
            .child(Radio::new("team").label("Team"))
            .child(Radio::new("enterprise").label("Enterprise").disabled(true))
            .on_click(cx.listener(|this, ix: &usize, _, cx| {
                this.plan = Some(*ix);
                cx.notify();
            }));

        let range = v_flex()
            .gap_3()
            .w(px(320.))
            .child(format!("Volume: {:.0}", self.volume_value))
            .child(Slider::new(&self.volume))
            .child(Progress::new("progress").value(self.volume_value));

        let select = Select::new(&self.language)
            .id("language")
            .title_prefix("Language: ")
            .w(px(240.));

        let tabs = v_flex()
            .gap_2()
            .child(
                TabBar::new("tabs")
                    .selected_index(self.tab)
                    .children([
                        Tab::new().label("General"),
                        Tab::new().label("Advanced"),
                        Tab::new().label("About"),
                    ])
                    .on_click(cx.listener(|this, index, _, cx| {
                        this.tab = *index;
                        cx.notify();
                    })),
            )
            .child(div().p_2().child(match self.tab {
                0 => "General settings go here.",
                1 => "Advanced settings go here.",
                _ => "GPUI controls demo, built with gpui-kit.",
            }));

        let plan = self
            .plan
            .and_then(|ix| ["Free", "Pro", "Team", "Enterprise"].get(ix).copied())
            .unwrap_or("none");
        let status_bar = h_flex()
            .gap_4()
            .px_4()
            .py_2()
            .text_sm()
            .text_color(cx.theme().muted_foreground)
            .border_t_1()
            .border_color(cx.theme().border)
            .child(self.status.clone())
            .child(format!("Agreed: {}", self.agreed))
            .child(format!("Plan: {plan}"));

        v_flex()
            .size_full()
            .bg(cx.theme().background)
            .text_color(cx.theme().foreground)
            .child(
                div()
                    .id("scroll")
                    .flex_1()
                    .overflow_y_scroll()
                    .child(
                        v_flex()
                            .gap_4()
                            .p_6()
                            .child(
                                div()
                                    .text_xl()
                                    .font_weight(FontWeight::BOLD)
                                    .child("GPUI Basic Controls"),
                            )
                            .child(Self::section("Buttons", cx, buttons))
                            .child(Self::section("Text input", cx, inputs))
                            .child(Self::section("Checkbox & switch", cx, toggles))
                            .child(Self::section("Radio group", cx, radios))
                            .child(Self::section("Slider & progress", cx, range))
                            .child(Self::section("Select (dropdown)", cx, select))
                            .child(Self::section("Tabs", cx, tabs)),
                    ),
            )
            .child(status_bar)
    }
}

fn main() {
    gpui_kit::application()
        .with_assets(gpui_kit::assets::Assets)
        .run(|cx| {
            gpui_kit::init(cx);
            let options = WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(Bounds::centered(
                    None,
                    size(px(820.), px(900.)),
                    cx,
                ))),
                ..Default::default()
            };
            gpui_kit::open_window(options, cx, |window, cx| {
                cx.new(|cx| ControlsDemo::new(window, cx))
            })
            .expect("failed to open window");
            cx.activate(true);
        });
}
