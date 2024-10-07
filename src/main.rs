#[derive(Debug)]


struct Usuario {

    nombre: String,
    email: String,
    edad: u8,
    activo: bool, 
    dni: u32, 
}

fn main() {


    let mut usuario1 = Usuario {
        nombre: String::from("davicho"),
        email: String::from("rodacri@gmail.com"),
        edad: 36,
        activo: true,
        dni: 5325255,
    };
    
    usuario1.email = String::from("zengolutil@gmail.com");
    
    let usuario2 = Usuario {
        nombre : String::from("Juan Roberto"),
        email : String::from("julio@gmail.com"),
        ..usuario1
    };


    println!("Here your struct: {:?}", usuario2);
    println!("Here your struct: {:#?}", nuevo_usuario(String::from("Julio Berne"),String::from("TC")));



}

fn nuevo_usuario (nombre: String, email: String) -> Usuario {
    Usuario {
        nombre,
        email,
        edad : 0,
        activo : true,
        dni : 443344384,
    }
}