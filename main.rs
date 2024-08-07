mod doenca;

fn main() {
    let gripe = doenca::Doenca::new(
        "Gripe".to_string(),
        vec!["Febre".to_string(), "Dor de garganta".to_string(), "Coriza".to_string()],
        "Vírus influenza".to_string(),
        "Repouso, medicamentos sintomáticos e hidratação".to_string(),
    );

    gripe.exibir();
}
