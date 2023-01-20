use std::collections::HashMap;

use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{
        canvas::{Canvas, Line},
        Block, Borders, List, ListItem, Paragraph,
    },
    Frame,
};

use crate::*;

pub fn ui<B>(
    f: &mut Frame<B>,
    size: usize,
    world: &World,
    selected_lf: Option<&LifeForm>,
    tick_rate: u64,
) where
    B: Backend,
{
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(0)
        .constraints([Constraint::Length(size as u16), Constraint::Min(20)].as_ref())
        .split(f.size());

    draw_main(f, size, selected_lf, tick_rate, world, chunks[0]);
    draw_controls(f, chunks[1]);
}

fn draw_main<B>(
    f: &mut Frame<B>,
    size: usize,
    selected_lf: Option<&LifeForm>,
    tick_rate: u64,
    world: &World,
    area: Rect,
) where
    B: Backend,
{
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .margin(0)
        .constraints([Constraint::Length(size as u16), Constraint::Min(10)].as_ref())
        .split(area);

    draw_world(f, size, selected_lf, world, chunks[0]);
    draw_right(f, selected_lf, tick_rate, world, chunks[1]);
}

fn draw_controls<B>(f: &mut Frame<B>, area: Rect)
where
    B: Backend,
{
    let block = Block::default().title("Controls").borders(Borders::ALL);
    let text = vec![Spans::from(
        "q = quit | p = pause | d = pause drawing | Up/Down = Select LifeForm | Left/Right = change tick rate",
    )];
    let paragraph = Paragraph::new(text).block(block);

    f.render_widget(paragraph, area);
}

fn draw_world<B>(
    f: &mut Frame<B>,
    size: usize,
    selected_lf: Option<&LifeForm>,
    world: &World,
    area: Rect,
) where
    B: Backend,
{
    let world_canvas = Canvas::default()
        .block(Block::default().title("World").borders(Borders::ALL))
        .x_bounds([0.0, size as f64])
        .y_bounds([0.0, size as f64])
        .paint(|ctx| {
            for water in &world.water {
                ctx.print(
                    water.0 as f64,
                    water.1 as f64,
                    Span::styled("W", Style::default().fg(Color::Blue)),
                );
            }

            for food in &world.food {
                ctx.print(
                    food.0 as f64,
                    food.1 as f64,
                    Span::styled("F", Style::default().fg(Color::Green)),
                );
            }

            for heal in &world.heals {
                ctx.print(
                    heal.0 as f64,
                    heal.1 as f64,
                    Span::styled("♥", Style::default().fg(Color::Red)),
                );
            }

            let mut num_at_location: HashMap<(usize, usize), usize> = HashMap::new();

            for lf in world.lifeforms.values() {
                *num_at_location.entry(lf.location).or_insert(0) += 1;
                let num = num_at_location[&lf.location];

                // Ideations on how to print the lifeforms once they have a direction
                // Ô (circumplex), O̺ (combined inverted bridge below), Ό (with tonos), O҉ (cryllic millions sign), O҈ (cryllic hundred thousands sign)
                // Oՙ (armenian half ring), O֑ (hebre etnahta), O֒ ,O֓ , O֔ , O֕ , ֕O, O֟, O֚   , O֛   O֣
                // ↘҉  , ↗, ↙, ↖,
                // Use arrows with the "combining cryllic millions sign (U+0489)", found here: https://www.fileformat.info/info/charset/UTF-8/list.htm?start=1024
                // TRIANGLES: ▲, ◥, ▶, ◢, ▼, ◣, ◀, ◤,
                //
                // TRIANGLES: ▲҉, ◥҉, ▶҉, ◢҉, ▼҉, ◣҉, ◀҉, ◤҉,

                let single_lf_char = match lf.orientation.name() {
                    DirectionName::North => "▲",
                    DirectionName::NorthEast => "◥",
                    DirectionName::East => "▶",
                    DirectionName::SouthEast => "◢",
                    DirectionName::South => "▼",
                    DirectionName::SouthWest => "◣",
                    DirectionName::West => "◀",
                    DirectionName::NorthWest => "◤",
                };

                let char = match num {
                    1 => single_lf_char,
                    2 => "2",
                    3 => "3",
                    4 => "4",
                    5 => "5",
                    6 => "6",
                    7 => "7",
                    8 => "8",
                    9 => "9",
                    _ => "!",
                };

                let color = match lf.health {
                    _ if lf.health >= 0.9 => Color::LightGreen,
                    _ if lf.health >= 0.8 => Color::Green,
                    _ if lf.health >= 0.7 => Color::LightBlue,
                    _ if lf.health >= 0.6 => Color::Blue,
                    _ if lf.health >= 0.5 => Color::Magenta,
                    _ if lf.health >= 0.4 => Color::LightMagenta,
                    _ if lf.health >= 0.3 => Color::Yellow,
                    _ if lf.health >= 0.2 => Color::LightYellow,
                    _ if lf.health >= 0.1 => Color::LightRed,
                    _ if lf.health < 0.1 => Color::Red,
                    _ => Color::White,
                };

                if let Some(selected_lf) = selected_lf {
                    if lf.id == selected_lf.id {
                        ctx.print(
                            lf.location.0 as f64,
                            lf.location.1 as f64,
                            Span::styled(char, Style::default().fg(Color::White)),
                        );
                    } else {
                        ctx.print(
                            lf.location.0 as f64,
                            lf.location.1 as f64,
                            Span::styled(char, Style::default().fg(color)),
                        );
                    }
                } else {
                    ctx.print(
                        lf.location.0 as f64,
                        lf.location.1 as f64,
                        Span::styled(char, Style::default().fg(color)),
                    );
                }
            }

            for danger in &world.danger {
                ctx.print(
                    danger.0 as f64,
                    danger.1 as f64,
                    Span::styled(
                        "☠ ",
                        Style::default()
                            .fg(Color::White)
                            .bg(Color::Red)
                            .add_modifier(Modifier::BOLD),
                    ),
                );
            }
        });

    f.render_widget(world_canvas, area);
}

fn draw_right<B>(
    f: &mut Frame<B>,
    selected_lf: Option<&LifeForm>,
    tick_rate: u64,
    world: &World,
    area: Rect,
) where
    B: Backend,
{
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(0)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(area);

    draw_top_right(f, tick_rate, world, chunks[0]);
    draw_single_lf_information(f, selected_lf, world, chunks[1]);
}

fn draw_single_lf_information<B>(
    f: &mut Frame<B>,
    selected_lf: Option<&LifeForm>,
    world: &World,
    area: Rect,
) where
    B: Backend,
{
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .margin(0)
        .constraints(
            [
                Constraint::Min(17),
                Constraint::Min(35),
                Constraint::Min(35),
                Constraint::Percentage(100),
            ]
            .as_ref(),
        )
        .split(area);

    draw_lf_selection(f, selected_lf, world, chunks[0]);
    draw_lf_input_neuron_values(f, selected_lf, chunks[1]);
    draw_lf_output_neuron_values(f, selected_lf, chunks[2]);
    draw_lf_neural_net(f, selected_lf, chunks[3]);
}

fn draw_lf_selection<B>(f: &mut Frame<B>, selected_lf: Option<&LifeForm>, world: &World, area: Rect)
where
    B: Backend,
{
    let items: Vec<ListItem> = world
        .lifeforms
        .values()
        .map(|lf| {
            if let Some(selected_lf) = selected_lf {
                if lf.id == selected_lf.id {
                    ListItem::new(format!("=> {}", lf.id)).style(
                        Style::default()
                            .bg(Color::White)
                            .fg(Color::Black)
                            .add_modifier(Modifier::BOLD),
                    )
                } else {
                    ListItem::new(format!("{}", lf.id))
                }
            } else {
                ListItem::new(format!("{}", lf.id))
            }
        })
        .collect();

    let list = List::new(items).block(
        Block::default()
            .title("Select LifeForm")
            .borders(Borders::ALL),
    );

    f.render_widget(list, area);
}

fn draw_lf_input_neuron_values<B>(f: &mut Frame<B>, selected_lf: Option<&LifeForm>, area: Rect)
where
    B: Backend,
{
    if let None = selected_lf {
        return;
    }

    let mut items: Vec<ListItem> = vec![];

    for (neuron_type, neuron) in selected_lf.unwrap().neural_net.input_neurons.values() {
        items.push(ListItem::new(format!(
            "{:?}: {:?}",
            neuron_type, neuron.value
        )));
    }

    let list = List::new(items).block(
        Block::default()
            .title(format!(
                "Input Neuron Values for {}",
                selected_lf.unwrap().id
            ))
            .borders(Borders::ALL),
    );

    f.render_widget(list, area);
}

fn draw_lf_output_neuron_values<B>(f: &mut Frame<B>, selected_lf: Option<&LifeForm>, area: Rect)
where
    B: Backend,
{
    if let None = selected_lf {
        return;
    }

    let values: &Vec<(OutputNeuronType, f32)>;

    if let None = selected_lf.unwrap().most_recent_output_neuron_values {
        return;
    } else {
        values = selected_lf
            .unwrap()
            .most_recent_output_neuron_values
            .as_ref()
            .unwrap();
    }

    let mut items: Vec<ListItem> = vec![];

    for (neuron_type, value) in values.iter() {
        items.push(ListItem::new(format!("{:?}: {}", neuron_type, value)));
    }

    let list = List::new(items).block(
        Block::default()
            .title("Output Neuron Values")
            .borders(Borders::ALL),
    );

    f.render_widget(list, area);
}

fn draw_lf_neural_net<B>(f: &mut Frame<B>, selected_lf: Option<&LifeForm>, area: Rect)
where
    B: Backend,
{
    if let None = selected_lf {
        return;
    }

    let lf = selected_lf.unwrap();

    // TODO This is gonna be friggin awesome
    // Only hard part is in representing neuron that connections to itself and two neurons that
    // connect back to each other.
    // Maybe to do Canvas' Points: https://docs.rs/tui/latest/tui/widgets/canvas/struct.Points.html
    // Good inf in here: https://docs.rs/tui/latest/src/tui/widgets/canvas/line.rs.html#16-57

    // TODO Get this outside of here and reassign to it instead of recreating a new one each time
    let neuron_locs = generate_neuron_hashmap(&lf.neural_net, &area);

    // Then for each genome, draw a line from each gene.from to gene.to
    // If it's a self reference... need a loop arrow, or just 3/4 or 4/5 of a circle
    let neural_net_canvas = Canvas::default()
        .block(Block::default().title("Neural Net").borders(Borders::ALL))
        .x_bounds([0.0, area.width as f64])
        .y_bounds([0.0, area.height as f64])
        .paint(|ctx| {
            // TODO I want to represent the weight AND the activity as well
            // There's an expressive greyscale here.
            // All connections could be in dark grey, then heavier ones could lighten?
            for (idx, gene) in lf.genome.genes.iter().enumerate() {
                let from = neuron_locs[&gene.from].1;
                let to = neuron_locs[&gene.to].1;
                ctx.draw(&Line {
                    x1: from.0,
                    y1: from.1,
                    x2: to.0,
                    y2: to.1,
                    color: Color::Rgb((idx * 10) as u8, (idx * 10) as u8, (idx * 10) as u8)
                    // color: Color::Yellow,
                });
                ctx.layer();
            }

            for (name, loc) in neuron_locs.values() {
                let x = (loc.0 - (name.len() / 2) as f64) + 1.0;
                ctx.print(x, loc.1, String::from(name))
            }
        });

    f.render_widget(neural_net_canvas, area);
}

fn draw_top_right<B>(f: &mut Frame<B>, tick_rate: u64, world: &World, area: Rect)
where
    B: Backend,
{
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .margin(0)
        .constraints([Constraint::Percentage(35), Constraint::Percentage(65)].as_ref())
        .split(area);

    draw_world_information(f, tick_rate, world, chunks[0]);
    draw_events(f, world, chunks[1]);
}

fn draw_world_information<B>(f: &mut Frame<B>, tick_rate: u64, world: &World, area: Rect)
where
    B: Backend,
{
    // TODO
    // * Dude the chart could be an amazing visualization for this, allowing us to see things
    // like average age over time
    // * Get oldest LF

    let block = Block::default()
        .borders(Borders::ALL)
        .title("World Information");

    let mut items: Vec<ListItem> = vec![];

    items.push(
        ListItem::new(format!(
            "Info: tick rate: {}ms | iteration: {}",
            tick_rate, world.tics
        ))
        .style(Style::default().fg(Color::Cyan)),
    );

    items.push(
        ListItem::new(format!("LifeForms: {}", world.lifeforms.len()))
            .style(Style::default().fg(Color::Green)),
    );

    let average_age: f32 = world
        .lifeforms
        .values()
        .map(|lf| lf.lifespan as f32)
        .sum::<f32>()
        / world.lifeforms.len() as f32;

    items.push(
        ListItem::new(format!("Avergae Age: {}", average_age))
            .style(Style::default().fg(Color::Green)),
    );

    let list = List::new(items).block(block);

    f.render_widget(list, area);
}

fn draw_events<B>(f: &mut Frame<B>, world: &World, area: Rect)
where
    B: Backend,
{
    let mut items: Vec<ListItem> = vec![];

    for (event_type, description) in &world.events {
        let color = match event_type {
            EventType::Death => Color::Blue,
            EventType::Creation => Color::Cyan,
            EventType::Mate => Color::Magenta,
            EventType::Attack => Color::Red,
            EventType::AsexuallyReproduce => Color::LightGreen,
        };

        items.insert(
            0,
            ListItem::new(Span::from(Span::styled(
                description,
                Style::default().fg(color),
            ))),
        );
    }

    let list = List::new(items).block(Block::default().title("Events").borders(Borders::ALL));

    f.render_widget(list, area);
}

/// Construct a hashmap of neuron_id => neuron location, used for drawing the neural net
fn generate_neuron_hashmap(
    neural_net: &NeuralNet,
    area: &Rect,
) -> HashMap<usize, (String, (f64, f64))> {
    let max_names_per_line = 7;

    // TODO Ok for all of three of these, gotta make it so there's a max of 7 per line, then
    // it moves onto a different line. So this *_spacing idea will be rethought.

    let input_neuron_spacing = area.width as f64 / (neural_net.input_neurons.len() + 1) as f64;
    let inner_neuron_spacing = area.width as f64 / (neural_net.inner_neurons.len() + 1) as f64;
    let output_neuron_spacing = area.width as f64 / (neural_net.output_neurons.len() + 1) as f64;

    let input_neuron_row = 1;
    let inner_neuron_row = (area.height / 2) + 2;
    let output_neuron_row = area.height - 1;

    let mut neuron_location_map = HashMap::new();

    for (idx, (neuron_type, neuron)) in neural_net.input_neurons.values().enumerate() {
        neuron_location_map.insert(
            neuron.id,
            (
                format!("{}", neuron_type),
                (
                    (idx + 1) as f64 * input_neuron_spacing,
                    input_neuron_row as f64,
                ),
            ),
        );
    }

    for (idx, neuron) in neural_net.inner_neurons.values().enumerate() {
        neuron_location_map.insert(
            neuron.id,
            (
                String::from("InnerNeuron"),
                (
                    (idx + 1) as f64 * inner_neuron_spacing,
                    inner_neuron_row as f64,
                ),
            ),
        );
    }

    for (idx, (neuron_type, neuron)) in neural_net.output_neurons.values().enumerate() {
        neuron_location_map.insert(
            neuron.id,
            (
                format!("{}", neuron_type),
                (
                    (idx + 1) as f64 * output_neuron_spacing,
                    output_neuron_row as f64,
                ),
            ),
        );
    }

    neuron_location_map
}
