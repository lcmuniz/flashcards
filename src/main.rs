mod domain;
mod storage;

fn main() {
    println!("Hello, world!");

    let mut flashcard = domain::Flashcard::novo("Pergunta 1", "Resposta 1").unwrap();
    println!("Flashcard criado: {:?}", flashcard);
    let storage = storage::FileStorage::novo("flashcards.json");
    storage.adicionar(&flashcard).unwrap();
    println!("Flashcard salvo em arquivo.");
    flashcard.atualizar_pergunta("Pergunta Atualizada").unwrap();
    storage.atualizar(&flashcard).unwrap();
    println!("Flashcard atualizado no arquivo.");
    storage.atualizar(&flashcard).unwrap();
    let flashcards = storage.listar().unwrap();
    println!("Flashcards carregados: {:?}", flashcards);
    storage.remover(flashcard.id).unwrap();
    println!("Flashcard removido do arquivo.");
    let flashcards = storage.listar().unwrap();
    println!("Flashcards restantes: {:?}", flashcards);
}
