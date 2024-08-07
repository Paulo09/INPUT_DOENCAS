
                                                                       INPUT DOENÇAS - RUST


1. Crie um novo arquivo chamado doenca.rs
2. Defina uma estrutura chamada Doenca, que contenha quatro campos: nome, sintomas, causa e tratamento.
3. Crie um método construtor para Doenca, que receba nome, sintomas, causa e tratamento como argumentos.
4. Crie métodos getter para os campos de Doenca.
5. Implemente o método de exibição para Doenca.
6. Adicione o código necessário ao main.rs para importar o módulo doenca.rs
7. Crie uma instância de Doenca e imprima-a.
Perguntas dessa tarefa
Crie o arquivo doenca.rs

Crie o arquivo main.rs

Mostrar rascunhos


Criando o Arquivo doenca.rs
Rust
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
Use o código com cuidado.

Criando o Arquivo main.rs
Rust
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
Use o código com cuidado.

Explicação:
Arquivo doenca.rs
Estrutura Doenca: Define os campos nome, sintomas, causa e tratamento.
Método new: É o construtor da estrutura, recebendo os valores para os campos e retornando uma nova instância de Doenca.
Métodos get_*: Retornam as referências imutáveis para os campos correspondentes, permitindo acesso aos dados da estrutura.
Método exibir: Formata e imprime as informações da doença de forma clara.
Arquivo main.rs
mod doenca;: Importa o módulo doenca para que possamos usar a estrutura Doenca neste arquivo.
Criando uma instância de Doenca: Cria uma nova instância da estrutura Doenca chamada gripe, passando os valores para os campos.
Chamando o método exibir: Imprime as informações da doença na tela.
Pontos importantes:
Ownership e empréstimo: Os métodos get_* retornam referências para evitar cópias desnecessárias e garantir a eficiência.
Formatação de saída: O método exibir utiliza println! e {:?} para formatar a saída de forma clara e concisa.
Vetores para sintomas: A lista de sintomas é representada por um vetor Vec<String> para permitir múltiplos sintomas.
Strings: Utilizamos String para armazenar texto mutável e &str para referências a partes de strings.
Rodando o código:
Salvar os arquivos: Salve os arquivos doenca.rs e main.rs no mesmo diretório.
Compilar e executar: Abra um terminal, navegue até o diretório onde você salvou os arquivos e execute o seguinte comando:
Bash
rustc main.rs
Use o código com cuidado.

Em seguida, execute o programa:
Bash
./main
Use o código com cuidado.

Isso irá imprimir as informações da doença na tela.

Este código fornece uma base sólida para representar doenças em Rust, permitindo a criação de estruturas mais complexas e a realização de operações adicionais com as informações das doenças.
