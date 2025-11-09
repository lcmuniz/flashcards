use crate::{domain::Flashcard, storage::FileStorage};
use std::io::{self, Write};

pub fn executar_cli() -> Result<(), Box<dyn std::error::Error>> {
    let storage = FileStorage::novo("flashcards.json");

    loop {
        println!("\n=== Aplicação de Flashcards ===");
        println!("1) Listar flashcards");
        println!("2) Criar novo flashcard");
        println!("3) Atualizar flashcard");
        println!("4) Remover flashcard");
        println!("5) Sair");

        print!("Escolha uma opção: ");
        io::stdout().flush()?;

        let mut opcao = String::new();
        io::stdin().read_line(&mut opcao)?;
        let opcao = opcao.trim();

        match opcao {
            "1" => listar(&storage)?,
            "2" => criar(&storage)?,
            "3" => atualizar(&storage)?,
            "4" => remover(&storage)?,
            "5" => {
                println!("Saindo...");
                break;
            }
            _ => println!("Opção inválida."),
        }
    }

    Ok(())
}

fn listar(storage: &FileStorage) -> Result<(), Box<dyn std::error::Error>> {
    let flashcards = storage.listar()?;
    if flashcards.is_empty() {
        println!("Nenhum flashcard encontrado.");
    } else {
        for (i, f) in flashcards.iter().enumerate() {
            println!("[{}] {} => {}", i + 1, f.pergunta, f.resposta);
        }
    }
    Ok(())
}

fn criar(storage: &FileStorage) -> Result<(), Box<dyn std::error::Error>> {
    let mut pergunta = String::new();
    let mut resposta = String::new();

    print!("Pergunta: ");
    io::stdout().flush()?;
    io::stdin().read_line(&mut pergunta)?;

    print!("Resposta: ");
    io::stdout().flush()?;
    io::stdin().read_line(&mut resposta)?;

    let flashcard = Flashcard::novo(&pergunta, &resposta)?;
    storage.adicionar(&flashcard)?;
    println!("Flashcard criado com sucesso!");
    Ok(())
}

// Lê uma linha com prompt mostrando o valor atual entre [valor_atual].
// Se o usuário só apertar Enter, retorna None (mantém o atual).
fn prompt_edit(label: &str, atual: &str) -> Result<Option<String>, io::Error> {
    print!("{label} [{atual}]: ");
    io::stdout().flush()?;
    let mut buf = String::new();
    let _ = io::stdin().read_line(&mut buf)?;
    let s = buf.trim().to_string();
    if s.is_empty() { Ok(None) } else { Ok(Some(s)) }
}

fn atualizar(storage: &FileStorage) -> Result<(), Box<dyn std::error::Error>> {
    let flashcards = storage.listar()?;
    if flashcards.is_empty() {
        println!("Não há flashcards para atualizar.");
        return Ok(());
    }

    // Mostra a lista e pede o índice
    listar(storage)?;
    print!("Digite o número do flashcard a atualizar: ");
    io::stdout().flush()?;

    let mut entrada = String::new();
    io::stdin().read_line(&mut entrada)?;
    let indice: usize = entrada.trim().parse().unwrap_or(0);

    if indice == 0 || indice > flashcards.len() {
        println!("Índice inválido.");
        return Ok(());
    }

    let mut flashcard = flashcards[indice - 1].clone();

    // Edita PERGUNTA (mantém se Enter)
    if let Some(nova_pergunta) = prompt_edit("Nova pergunta", &flashcard.pergunta)? {
        // validação via domínio
        flashcard.atualizar_pergunta(&nova_pergunta)?;
    }

    // Edita RESPOSTA (mantém se Enter)
    if let Some(nova_resposta) = prompt_edit("Nova resposta", &flashcard.resposta)? {
        flashcard.atualizar_resposta(&nova_resposta)?;
    }

    // Persiste alterações
    storage.atualizar(&flashcard)?;
    println!("Flashcard atualizado!");
    Ok(())
}

fn remover(storage: &FileStorage) -> Result<(), Box<dyn std::error::Error>> {
    let flashcards = storage.listar()?;
    if flashcards.is_empty() {
        println!("Não há flashcards para remover.");
        return Ok(());
    }

    listar(storage)?;
    print!("Digite o número do flashcard a remover: ");
    io::stdout().flush()?;

    let mut entrada = String::new();
    io::stdin().read_line(&mut entrada)?;
    let indice: usize = entrada.trim().parse().unwrap_or(0);

    if indice == 0 || indice > flashcards.len() {
        println!("Índice inválido.");
        return Ok(());
    }

    let id = flashcards[indice - 1].id;
    storage.remover(id)?;
    println!("Flashcard removido.");
    Ok(())
}
