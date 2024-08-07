struct Doenca {
    nome: String,
    sintomas: Vec<String>,
    causa: String,
    tratamento: String,
}

impl Doenca {
    fn new(nome: String, sintomas: Vec<String>, causa: String, tratamento: String) -> Self {
        Doenca {
            nome,
            sintomas,
            causa,
            tratamento,
        }
    }

    fn get_nome(&self) -> &str {
        &self.nome
    }

    fn get_sintomas(&self) -> &Vec<String> {
        &self.sintomas
    }

    fn get_causa(&self) -> &str {
        &self.causa
    }

    fn get_tratamento(&self) -> &str {
        &self.tratamento
    }

    fn exibir(&self) {
        println!("Doença: {}", self.nome);
        println!("Sintomas: {:?}", self.sintomas);
        println!("Causa: {}", self.causa);
        println!("Tratamento: {}", self.tratamento);
    }
}
