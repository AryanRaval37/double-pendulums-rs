use nannou::prelude::*;

const G: f64 = 9.806;
const FIXED_DT: f64 = 0.0001;
const PIXELS_PER_METER: f32 = 150.0;

struct Model {
    theta1: f64, // upper rod angle (rad)
    theta2: f64, // lower rod angle (rad)
    omega1: f64, // upper angular velocity (rad/s)
    omega2: f64, // lower angular velocity (rad/s)

    m1: f64, // upper bob mass (kg)
    m2: f64, // lower bob mass (kg)
    l1: f64, // upper rod length (m)
    l2: f64, // lower rod length (m)

    accumulator: f64, // unspent real time (s)
}

fn main() {
    nannou::app(model).simple_window(view).update(update).run();
}

fn model(_app: &App) -> Model {
    Model {
        // Start with both rods horizontal and at rest
        theta1: std::f64::consts::FRAC_PI_2,
        theta2: std::f64::consts::FRAC_PI_2,
        omega1: 0.0,
        omega2: 0.0,

        m1: 4.0, 
        m2: 1.0, 
        l1: 1.5, 
        l2: 1.0, 

        accumulator: 0.0,
    }
}

// State vector `[theta1, omega1, theta2, omega2]` used by the integrator.
type State = [f64; 4];

fn derivatives(s: &State, m: &Model) -> State {
    let (t1, w1, t2, w2) = (s[0], s[1], s[2], s[3]);
    let (m1, m2, l1, l2) = (m.m1, m.m2, m.l1, m.l2);

    let delta = t1 - t2;
    let denom = 2.0 * m1 + m2 - m2 * (2.0 * t1 - 2.0 * t2).cos();

    let alpha1 = (-G * (2.0 * m1 + m2) * t1.sin()
        - m2 * G * (t1 - 2.0 * t2).sin()
        - 2.0 * delta.sin() * m2 * (w2 * w2 * l2 + w1 * w1 * l1 * delta.cos()))
        / (l1 * denom);

    let alpha2 = (2.0
        * delta.sin()
        * (w1 * w1 * l1 * (m1 + m2)
            + G * (m1 + m2) * t1.cos()
            + w2 * w2 * l2 * m2 * delta.cos()))
        / (l2 * denom);

    [w1, alpha1, w2, alpha2]
}

// Returns `a + b * scale`, component-wise.
fn add_scaled(a: &State, b: &State, scale: f64) -> State {
    [
        a[0] + b[0] * scale,
        a[1] + b[1] * scale,
        a[2] + b[2] * scale,
        a[3] + b[3] * scale,
    ]
}

// use RK4 integration
fn step_physics(m: &mut Model, dt: f64) {
    let s0: State = [m.theta1, m.omega1, m.theta2, m.omega2];

    let k1 = derivatives(&s0, m);
    let k2 = derivatives(&add_scaled(&s0, &k1, dt / 2.0), m);
    let k3 = derivatives(&add_scaled(&s0, &k2, dt / 2.0), m);
    let k4 = derivatives(&add_scaled(&s0, &k3, dt), m);

    let step = dt / 6.0;
    m.theta1 += step * (k1[0] + 2.0 * k2[0] + 2.0 * k3[0] + k4[0]);
    m.omega1 += step * (k1[1] + 2.0 * k2[1] + 2.0 * k3[1] + k4[1]);
    m.theta2 += step * (k1[2] + 2.0 * k2[2] + 2.0 * k3[2] + k4[2]);
    m.omega2 += step * (k1[3] + 2.0 * k2[3] + 2.0 * k3[3] + k4[3]);
}

fn update(app: &App, m: &mut Model) {
    m.accumulator += app.time_delta() as f64;

    // Clamp to avoid a "spiral of death" if the app stalls (e.g. window drag).
    if m.accumulator > 0.25 {
        m.accumulator = 0.25;
    }

    while m.accumulator >= FIXED_DT {
        step_physics(m, FIXED_DT);
        m.accumulator -= FIXED_DT;
    }
}

fn view(app: &App, model: &Model, _view: Entity) {
    let draw = app.draw();
    draw.background().color(WHITE);

    // Pivot: fixed point near the top-centre of the window.
    let pivot = app.window_rect().pad_top(150.0).mid_top();

    // Bob positions in metres (physics space, +y up, origin at the pivot),
    // then scaled to pixels. A rod hanging straight down (theta = 0) points to
    // -y, i.e. below the pivot on screen.
    let (t1, t2) = (model.theta1, model.theta2);
    let x1 = (model.l1 * t1.sin()) as f32;
    let y1 = (-model.l1 * t1.cos()) as f32;
    let x2 = x1 + (model.l2 * t2.sin()) as f32;
    let y2 = y1 - (model.l2 * t2.cos()) as f32;

    let bob1 = pivot + vec2(x1, y1) * PIXELS_PER_METER;
    let bob2 = pivot + vec2(x2, y2) * PIXELS_PER_METER;

    // Visual bob radius scales with the cube root of mass (constant density
    // sphere), so relative sizes read correctly.
    let r1 = 12.0 * (model.m1 as f32).cbrt();
    let r2 = 12.0 * (model.m2 as f32).cbrt();

    draw.line()
        .start(pivot)
        .end(bob1)
        .stroke_weight(3.0)
        .color(GRAY);
    draw.line()
        .start(bob1)
        .end(bob2)
        .stroke_weight(3.0)
        .color(GRAY);

    draw.ellipse().xy(bob1).radius(r1).color(BLACK);
    draw.ellipse().xy(bob2).radius(r2).color(BLACK);
}
