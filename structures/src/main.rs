struct User {
    first_name: String,
    last_name: String,
    email: String,
    active: bool,
    sign_in_count: u32,
}

// tupple struct
struct Coordonates(f64, f64);
struct Color(u32, u32, u32);
struct CoordonatesColor {
    coordonates: Coordonates,
    color: Color,
}

fn main() {
    let mut user = User {
        first_name: String::from("Osiac"),
        last_name: String::from("Victor"),
        email: String::from("email@example.com"),
        active: true,
        sign_in_count: 1,
    };

    println!("username email {}", user.email);
    user.email = String::from("changed_email@example.com");
    println!("changed username email {}", user.email);

    let builded_user_by_params = build_user(
        String::from("gigi"),
        String::from("kent"),
        String::from("gigi.kent@example.com"),
    );
    println!(
        "Builded user name: {} {}",
        builded_user_by_params.first_name, builded_user_by_params.last_name
    );

    let mut builded_user_shorthand = build_user_shorthand(
        String::from("mardare"),
        String::from("ciufutu"),
        String::from("mardare@example.com"),
    );
    println!(
        "Builded user name: {} {}",
        builded_user_shorthand.first_name, builded_user_shorthand.last_name
    );

    let updated_user = update_struct(builded_user_shorthand);
    println!("updated user email: {}", updated_user.email);
    println!(
        "updated user name: {} {}",
        updated_user.first_name, updated_user.last_name
    );

    let coordonates_color = init_coordonates_color(init_coordonates(), init_color());
    println!("Longitude {}", coordonates_color.coordonates.0);
    println!(
        "Longitude color ({} {} {})",
        coordonates_color.color.0, coordonates_color.color.1, coordonates_color.color.2
    );
}

fn build_user(first_name: String, last_name: String, email: String) -> User {
    User {
        first_name: first_name,
        last_name: last_name,
        email: email,
        active: true,
        sign_in_count: 1,
    }
}

// params must match field names
fn build_user_shorthand(first_name: String, last_name: String, email: String) -> User {
    User {
        first_name,
        last_name,
        email,
        active: true,
        sign_in_count: 1,
    }
}

// struct update

fn update_struct(user: User) -> User {
    User {
        email: String::from("from_update_sctruct@example.com"),
        ..user
    }
}

fn init_color() -> Color {
    Color(0, 34, 55)
}

fn init_coordonates() -> Coordonates {
    Coordonates(45.085, 23.099)
}

fn init_coordonates_color(coordonates: Coordonates, color: Color) -> CoordonatesColor {
    CoordonatesColor { coordonates, color }
}
