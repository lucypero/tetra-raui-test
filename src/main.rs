use raui::core::{prelude::*, widget::setup as setup_core};
use raui_tetra_renderer::simple_host::{PreloadedFont, TetraSimpleHost};
use tetra::graphics::{self};
use tetra::{Context, ContextBuilder, State};

const FONT_SIZE:usize = 18;

const BG_COLOR: Color = Color { r: 0.878, g: 0.894, b: 0.839, a: 1.};

// turquoise
//BUTTON ACTIVE colors
const BUTTON_BG_COLOR_ACTIVE: Color = Color { r: 0.549, g: 0.674, b: 0., a: 1. };
const BUTTON_FG_COLOR_ACTIVE: Color = Color { r: 1., g: 1., b: 1., a: 1.};
//

//BUTTON INACTIVE colors
const BUTTON_BG_COLOR_INACTIVE: Color = Color { r: 0.549, g: 0.674, b: 0., a: 0. };
const BUTTON_FG_COLOR_INACTIVE: Color = Color { r: 0., g: 0., b: 0., a: 1.};

struct GameState {
    ui: TetraSimpleHost,
}

fn setup(app: &mut Application) {
    app.setup(setup_core);
}

impl GameState {
    fn new(ctx: &mut Context) -> tetra::Result<Self> {
        let preload_fonts = vec![PreloadedFont {
            id: "font0",
            size: FONT_SIZE,
            scale: 1.,
            path: "font.ttf",
        }];

        let preload_textures = vec![];

        let tree = widget! {(app)};
        let ui = TetraSimpleHost::new(ctx, tree, &preload_fonts, &preload_textures, setup)?;

        println!("fonts {:?}", ui.resources.fonts);

        Ok(GameState { ui })
    }
}

// mark the root widget as navigable container to allow button to subscribe to navigation system.
#[pre_hooks(use_nav_container_active)]
pub fn app(mut _ctx: WidgetContext) -> WidgetNode {
    // let WidgetContext {
    //     id,
    //     key,
    //     props,
    //     state,
    //     ..
    // } = ctx;

    //TODO(lucypero): buttons:
    // https://github.com/RAUI-labs/raui/blob/master/examples/button_internal.rs


    let button = make_widget!(button)
        // enable button navigation (it is disabled by default).
        .with_props(NavItemActive)
        // by default button state of the button is passed to the content widget with
        // `ButtonProps` props data, so content widget can read it and change its appearance.
        .named_slot("content", make_widget!(button_internal));

    let rect = make_widget!(image_box)
        .with_props(ImageBoxProps::colored(raui::prelude::Color {
            r: 1.0,
            g: 0.25,
            b: 0.25,
            a: 1.0,
        }))
        .with_props(FlexBoxItemLayout {
            margin: 10.0.into(),
            ..Default::default()
        });

    make_widget!(size_box)
        .with_props(SizeBoxProps {
            width: SizeBoxSizeValue::Exact(400.0),
            height: SizeBoxSizeValue::Exact(300.0),
            margin: 100.0.into(),
            ..Default::default()
        })
        //background color of the menu
        .named_slot(
            "content",
            make_widget!(content_box)
                .listed_slot(make_widget!(image_box).with_props(ImageBoxProps::colored(
                    raui::prelude::Color {
                        r: 0.0,
                        g: 0.25,
                        b: 1.0,
                        a: 1.0,
                    },
                )))
                .listed_slot(
                    make_widget!(flex_box)
                        .with_props(FlexBoxProps {
                            direction: FlexBoxDirection::VerticalBottomToTop,
                            ..Default::default()
                        })
                        .listed_slot(button.clone())
                        .listed_slot(button.clone())
                        .listed_slot(button.clone())
                ),
        )
        .into()
}

fn button_internal(ctx: WidgetContext) -> WidgetNode {
    // first we unpack button state from button props.
    let ButtonProps {
        // selected state means, well..widget has got selected. selection in navigation is more
        // complex than that and it deserves separate deeper explanation, but in essence: whenever
        // user navigate over the UI, RAUI performs selection on navigable items, navigable items
        // may be nested and whenever some widget gets selected, all of its navigable parents
        // receive selection event too, so there is not only one widget that might be selected at
        // a time, but there might be a chain of selected items, as long as they are on the way
        // toward actually selected navigable item in the widget tree.
        selected,
        // trigger state means navigable item got Accept event, which in context of the button
        // means: button is selected and user performed "left mouse button click".
        trigger,
        // context state is similar to trigger state, in this case it means user performed "right
        // mouse button click".
        context,
        ..
    } = ctx.props.read_cloned_or_default();

    let color = if trigger {
        Color {
            r: 1.0,
            g: 0.25,
            b: 0.25,
            a: 1.0,
        }
    } else if context {
        Color {
            r: 0.25,
            g: 1.0,
            b: 0.25,
            a: 1.0,
        }
    } else if selected {
        Color {
            r: 0.25,
            g: 0.25,
            b: 1.0,
            a: 1.0,
        }
    } else {
        Color {
            r: 0.25,
            g: 0.25,
            b: 0.25,
            a: 1.0,
        }
    };

    if trigger {
        println!("button clicked");
    }

    if context {
        println!("context");
    }

    if selected {
        println!("selected");
    }
    
    //button could be
    // content_box -> text_box
    //    -> image_box
    //    -> text_box

    make_widget!(content_box)
        .listed_slot(make_widget!(image_box)
            .with_props(ImageBoxProps::colored(
                BUTTON_BG_COLOR_ACTIVE
            )))
        .listed_slot(make_widget!(text_box))
            .with_props(TextBoxProps{
                text: "Start".to_owned(),
                font: TextBoxFont {
                    name: "font0".to_owned(),
                    size: FONT_SIZE as f32,
                },
                color: BUTTON_FG_COLOR_ACTIVE,
                ..Default::default()
            })
        .into()



    // make_widget!(image_box)
    //     .with_props(ImageBoxProps {
    //         material: ImageBoxMaterial::Color(ImageBoxColor {
    //             color,
    //             ..Default::default()
    //         }),
    //         width: ImageBoxSizeValue::Exact(25.0),
    //         height: ImageBoxSizeValue::Exact(25.0),
    //         ..Default::default()
    //     })
    //     .into()
}

impl State for GameState {
    fn update(&mut self, context: &mut Context) -> tetra::Result {
        self.ui.update(context);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        graphics::clear(ctx, tetra::graphics::Color::rgb(0.392, 0.584, 0.929));
        self.ui.draw(ctx, PrintLogger)?;
        Ok(())
    }

    fn event(&mut self, ctx: &mut Context, event: tetra::Event) -> tetra::Result {
        self.ui.event(ctx, &event);
        Ok(())
    }
}

fn main() -> tetra::Result {
    ContextBuilder::new("Tetra-RAUI test", 1280, 720)
        .show_mouse(true)
        .build()?
        .run(|ctx| GameState::new(ctx))
}
