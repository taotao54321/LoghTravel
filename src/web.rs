use seed::{prelude::*, *};

use crate::planet::{planet_name, planet_position, PLANET_COUNT};
use crate::query::{Answer, Query, QueryPlanet, QueryPosition};

#[wasm_bindgen(start)]
pub fn start() {
    App::start("app", init, update, view);
}

const SPEEDS: [u32; 5] = [30, 20, 16, 12, 10];

const ENERGY_MAX: u32 = 100;

#[derive(Debug)]
struct Model {
    speed: u32,
    query: Query,
}

impl Default for Model {
    fn default() -> Self {
        Self {
            speed: SPEEDS[0],
            query: Default::default(),
        }
    }
}

#[derive(Debug)]
enum Msg {
    SetSpeed(u32),
    SetSourcePlanet(usize),
    SetSourcePosition,

    SetQueryPlanetEnergy(u32),
    SetQueryPositionX(u32),
    SetQueryPositionY(u32),
    SetQueryPositionZ(u32),
}

fn init(_url: Url, _orders: &mut impl Orders<Msg>) -> Model {
    Model::default()
}

fn update(msg: Msg, model: &mut Model, _orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::SetSpeed(speed) => model.speed = speed,
        Msg::SetSourcePlanet(src) => match &mut model.query {
            Query::Planet(q) => q.set_src(src),
            Query::Position(_) => model.query = Query::new_planet(src, ENERGY_MAX),
        },
        Msg::SetSourcePosition => {
            if matches!(model.query, Query::Planet(_)) {
                model.query = Query::default_position();
            }
        }
        Msg::SetQueryPlanetEnergy(energy) => {
            if let Query::Planet(q) = &mut model.query {
                q.set_energy(energy);
            }
        }
        Msg::SetQueryPositionX(x) => {
            if let Query::Position(q) = &mut model.query {
                q.set_src_x(x);
            }
        }
        Msg::SetQueryPositionY(y) => {
            if let Query::Position(q) = &mut model.query {
                q.set_src_y(y);
            }
        }
        Msg::SetQueryPositionZ(z) => {
            if let Query::Position(q) = &mut model.query {
                q.set_src_z(z);
            }
        }
    }
}

fn view(model: &Model) -> Node<Msg> {
    let ans = model.query.execute();

    div![
        view_speed(model),
        view_query(model),
        div![
            style! {
                St::Display => "flex",
                St::FlexDirection => "row",
            },
            view_map(model, &ans),
            view_table(model, &ans),
        ],
    ]
}

fn view_speed(model: &Model) -> Node<Msg> {
    const ID_SELECT: &str = "select-speed";

    let options = SPEEDS.iter().map(|&speed| {
        option![
            attrs! {
                At::Value => speed,
                At::Selected => (speed == model.speed).as_at_value(),
            },
            speed,
        ]
    });

    p![
        label![
            attrs! {
                At::For => ID_SELECT,
            },
            "移動力: ",
        ],
        select![
            id!(ID_SELECT),
            options,
            input_ev(Ev::Change, |s| s.parse::<u32>().ok().map(Msg::SetSpeed)),
        ],
    ]
}

fn view_query(model: &Model) -> Node<Msg> {
    const ID_SELECT: &str = "select-query";
    const VALUE_POSITION: usize = 999;

    let options_planet = (0..PLANET_COUNT).map(|id| {
        let name = planet_name(id);
        option![
            attrs! {
                At::Value => id,
            },
            format!("{id}:{name}"),
        ]
    });

    let option_pos = option![
        attrs! {
            At::Value => VALUE_POSITION,
        },
        "座標指定",
    ];

    let input_args = match &model.query {
        Query::Planet(q) => view_query_planet(q),
        Query::Position(q) => view_query_position(q),
    };

    let value = match &model.query {
        Query::Planet(q) => q.src(),
        Query::Position(_) => VALUE_POSITION,
    };

    p![
        label![
            attrs! {
                At::For => ID_SELECT,
            },
            "始点: ",
        ],
        select![
            id!(ID_SELECT),
            attrs! {
                At::Value => value,
            },
            options_planet,
            option_pos,
            input_ev(Ev::Change, move |s| s.parse::<usize>().ok().and_then(
                |x| match x {
                    0..=PLANET_COUNT => Some(Msg::SetSourcePlanet(x)),
                    VALUE_POSITION => Some(Msg::SetSourcePosition),
                    _ => None,
                }
            )),
        ],
        input_args,
    ]
}

fn view_query_planet(q: &QueryPlanet) -> Node<Msg> {
    const ID_INPUT: &str = "input-query-planet-energy";

    span![
        label![
            attrs! {
                At::For => ID_INPUT,
            },
            "エネルギー: "
        ],
        input![
            id!(ID_INPUT),
            attrs! {
                At::Type => "number",
                At::Min => 0,
                At::Max => ENERGY_MAX,
                At::Value => q.energy(),
            },
            input_ev(Ev::Change, |s| s
                .parse::<u32>()
                .ok()
                .map(Msg::SetQueryPlanetEnergy)),
        ],
    ]
}

fn view_query_position(q: &QueryPosition) -> Node<Msg> {
    span![
        label!["座標: "],
        "(",
        view_position_component(q.src().x, Msg::SetQueryPositionX),
        ",",
        view_position_component(q.src().y, Msg::SetQueryPositionY),
        ",",
        view_position_component(q.src().z, Msg::SetQueryPositionZ),
        ")",
    ]
}

fn view_position_component<F>(value: u32, f: F) -> Node<Msg>
where
    F: FnOnce(u32) -> Msg + Clone + 'static,
{
    input![
        attrs! {
            At::Type => "number",
            At::Min => 0,
            At::Max => 128,
            At::Value => value,
        },
        input_ev(Ev::Change, |s| s.parse::<u32>().ok().map(f)),
    ]
}

fn view_table(model: &Model, ans: &Answer) -> Node<Msg> {
    let rows = (0..PLANET_COUNT)
        .filter(|&dst| !planet_is_source(model, dst))
        .flat_map(|dst| {
            ans.cost(dst, model.speed).map(|(turn, energy)| {
                let name = planet_name(dst);
                let dist = model.query.src_pos().distance(planet_position(dst));
                tr![
                    td![format!("{dst}:{name}")],
                    td![dist],
                    td![turn],
                    td![energy],
                ]
            })
        });

    div![
        style! {
            St::AlignSelf => "start",
        },
        table![
            thead![tr![th!["目的地"], th!["距離"], th!["日数"], th!["消費"]],],
            tbody![rows],
        ],
    ]
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum PlanetVolume {
    Large,
    Medium,
    Small,
    Tiny,
}

impl PlanetVolume {
    fn iter() -> impl Iterator<Item = Self> + Clone {
        use PlanetVolume::*;

        [Large, Medium, Small, Tiny].into_iter()
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum PlanetColor {
    Source,
    Reachable,
    Unreachable,
}

impl PlanetColor {
    fn iter() -> impl Iterator<Item = Self> + Clone {
        use PlanetColor::*;

        [Source, Reachable, Unreachable].into_iter()
    }
}

fn view_map(model: &Model, ans: &Answer) -> Node<Msg> {
    const WIDTH: u32 = 512;
    const HEIGHT: u32 = 480;

    let defs_images =
        itertools::iproduct!(PlanetVolume::iter(), PlanetColor::iter()).map(|(volume, color)| {
            let image_id = planet_image_id(volume, color);
            image![
                id!(&image_id),
                attrs! {
                    At::Href => format!("asset/{image_id}.png"),
                    At::Width => 32,
                    At::Height => 32,
                },
            ]
        });

    let planet_uses = (0..PLANET_COUNT).map(|id| {
        let volume = planet_volume(id);
        let color = match (planet_is_source(model, id), ans.is_reachable(id)) {
            (true, _) => PlanetColor::Source,
            (false, true) => PlanetColor::Reachable,
            (false, false) => PlanetColor::Unreachable,
        };
        let (x, y) = planet_position_on_map(id);
        r#use![
            attrs! {
                At::Href => format!("#{}", planet_image_id(volume, color)),
                At::X => x,
                At::Y => y,
            },
            ev(Ev::Click, move |_| Msg::SetSourcePlanet(id)),
        ]
    });

    let planet_turns = (0..PLANET_COUNT)
        .filter(|&dst| !planet_is_source(model, dst))
        .flat_map(|dst| {
            if planet_is_source(model, dst) {
                return None;
            }
            ans.cost(dst, model.speed).map(|(turn, _)| {
                let (px, py) = planet_position_on_map(dst);
                let x = px + 12;
                let y = py + 48;
                text![
                    attrs! {
                        At::X => x,
                        At::Y => y,
                        At::Fill => "yellow",
                    },
                    turn,
                ]
            })
        });

    svg![
        attrs! {
            At::Width => WIDTH,
            At::Height => HEIGHT,
            At::ViewBox => format!("0 0 {WIDTH} {HEIGHT}"),
        },
        defs![defs_images],
        image![attrs! {
            At::Href => "asset/map-background.png",
            At::Width => WIDTH,
            At::Height => HEIGHT,
        }],
        planet_uses,
        planet_turns,
    ]
}

fn planet_image_id(volume: PlanetVolume, color: PlanetColor) -> String {
    let s_volume = match volume {
        PlanetVolume::Large => "large",
        PlanetVolume::Medium => "medium",
        PlanetVolume::Small => "small",
        PlanetVolume::Tiny => "tiny",
    };

    let s_color = match color {
        PlanetColor::Source => "blue",
        PlanetColor::Reachable => "red",
        PlanetColor::Unreachable => "gray",
    };

    format!("planet-{s_volume}-{s_color}")
}

fn planet_volume(id: usize) -> PlanetVolume {
    const TABLE: [PlanetVolume; PLANET_COUNT] = [
        PlanetVolume::Large,
        PlanetVolume::Large,
        PlanetVolume::Large,
        PlanetVolume::Medium,
        PlanetVolume::Large,
        PlanetVolume::Tiny,
        PlanetVolume::Medium,
        PlanetVolume::Medium,
        PlanetVolume::Large,
        PlanetVolume::Small,
        PlanetVolume::Medium,
        PlanetVolume::Large,
        PlanetVolume::Tiny,
        PlanetVolume::Tiny,
        PlanetVolume::Large,
        PlanetVolume::Large,
        PlanetVolume::Tiny,
        PlanetVolume::Small,
        PlanetVolume::Medium,
        PlanetVolume::Medium,
        PlanetVolume::Tiny,
        PlanetVolume::Small,
        PlanetVolume::Medium,
        PlanetVolume::Tiny,
        PlanetVolume::Tiny,
        PlanetVolume::Small,
        PlanetVolume::Tiny,
        PlanetVolume::Tiny,
        PlanetVolume::Tiny,
        PlanetVolume::Tiny,
        PlanetVolume::Tiny,
        PlanetVolume::Tiny,
    ];

    TABLE[id]
}

fn planet_position_on_map(id: usize) -> (u32, u32) {
    const TABLE: [(u32, u32); PLANET_COUNT] = [
        (56, 392),
        (56, 344),
        (104, 360),
        (72, 296),
        (152, 280),
        (200, 216),
        (104, 216),
        (232, 168),
        (280, 312),
        (168, 168),
        (232, 280),
        (328, 248),
        (56, 248),
        (56, 184),
        (424, 392),
        (440, 296),
        (184, 328),
        (248, 40),
        (376, 216),
        (376, 168),
        (248, 104),
        (328, 168),
        (392, 328),
        (296, 120),
        (312, 376),
        (392, 120),
        (360, 72),
        (440, 360),
        (376, 392),
        (424, 232),
        (424, 168),
        (424, 40),
    ];

    TABLE[id]
}

fn planet_is_source(model: &Model, id: usize) -> bool {
    match &model.query {
        Query::Planet(q) => id == q.src(),
        _ => false,
    }
}
