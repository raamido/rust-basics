fn main() {
    /*  Enumeration  */

    enum Roles {
        User,
        Admin,
        SuperAdmin,
    }

    fn match_role(role: Roles) {
        match role {
            Roles::User => println!("User"),
            Roles::Admin => println!("Admin"),
            Roles::SuperAdmin => println!("Super Admin"),
        }
    }

    match_role(Roles::User);
    match_role(Roles::Admin);
    match_role(Roles::SuperAdmin);
}
