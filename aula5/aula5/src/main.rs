#[derive(Debug)]
struct Node {
    value: i32,               // Valor do nó
    left: Option<Box<Node>>,  // Filho esquerdo
    right: Option<Box<Node>>, // Filho direito
}

// Árvore BST
#[derive(Debug)]
struct BST {
    root: Option<Box<Node>>, // Raiz da árvore
}

impl BST {
    // Criar árvore vazia
    fn new() -> Self {
        BST { root: None }
    }

    // Inserir valor
    fn insert(&mut self, value: i32) {
        let mut current = &mut self.root;
        while let Some(node) = current {
            if value < node.value {
                // Quando o valor é menor, vai para a esquerda
                current = &mut node.left;
            } else if value > node.value {
                // Quando o valor é maior, vai para a direita
                current = &mut node.right;
            } else {
                // Se o valor for igual, não faz nada (não permite duplicatas)
                return;
            }
        }

        // Quando encontrar uma posição vazia, insere o nó
        *current = Some(Box::new(Node {
            value,
            left: None,
            right: None,
        }));
    }

    // Buscar valor
    fn search(&self, value: i32) -> bool {
        let mut current = &self.root;
        while let Some(node) = current {
            if value == node.value {
                return true;
            } else if value < node.value {
                current = &node.left;
            } else {
                current = &node.right;
            }
        }
        false
    }

    // Função para imprimir a árvore de forma legível
    fn print_tree(&self) {
        println!("Estrutura da Árvore:");
        self.print_node(&self.root, 0);
    }

    // Função auxiliar para impressão recursiva
    fn print_node(&self, node: &Option<Box<Node>>, depth: usize) {
        if let Some(n) = node {
            // Imprimir o valor do nó com o número de espaços representando a profundidade
            println!("{:indent$}{}", "", n.value, indent = depth * 4);
            self.print_node(&n.left, depth + 1);   // Imprime o filho esquerdo
            self.print_node(&n.right, depth + 1);  // Imprime o filho direito
        }
    }
}

fn main() {
    println!("=== Árvore Binária de Busca ===");

    // Criar árvore e inserir valores
    let mut bst = BST::new();
    bst.insert(10);
    bst.insert(27);
    bst.insert(23);
    bst.insert(7);

    // Imprimir a árvore para debugar a estrutura
    bst.print_tree();

    // Buscar valores
    println!("\n10 existe na BST? {}", bst.search(10));  // true
    println!("7 existe na BST? {}", bst.search(7));    // true
    println!("24 existe na BST? {}", bst.search(24));  // false
}
