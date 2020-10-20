Projeto: Construindo um programa de linha de comando
*****************************************************

Projeto descrito na documentação da linguagem Rust com comentários
a partir das explicações.

https://doc.rust-lang.org/book/ch12-00-an-io-project.html


Considerações de Modularidade e Tratamento de Erro
===================================================
#. Evitar da função *main* ter mais que uma responsabilidade
#. agrupar variáveis com um certo propósito em estruturas
#. Se preocupar se o erro retornado é explicativo
#. Concentrar o tratamento de erro num único lugar

Separação dos binários
=======================
#. Mover a lógica para *lib.rs*
#. Partes pequenas de código podem permanecer em *main.rs*
#. Quando se tornar complexo extrair de *main.rs* para *lib.rs*

Responsabilidades em main.rs
==============================
* Chamar lógica passando os argumentos
* Setar alguma configuração
* Chamar função run em *lib.rs*
* Tratar erro caso run retornar um erro

Test-driven development(TDD)
=============================
1) Escrever um teste que falhe e receber a falha esperada ao rodar
2) Modificar para um teste que retorne sucesso
3) Refatorar o código e verificar que os testes continuem passando
4) Repetir do passo 1

Ler argumentos 
===============
.. code-block:: rust

    let args: Vec<String> = env::args().collect();

Obs: Chamada *env::args* ao invés de *args* evita ambiguidade pois indica o módulo

Lendo um arquivo
=================
.. code-block:: rust

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

Agrupar variáveis numa estrutura
=================================
.. code-block:: rust

    struct Config {
        query: String,
        filename: String,
    }

    impl Config {
        fn new(args: &[String]) -> Config {
            if args.len() < 3 {
                panic!("not enough arguments");
            }

            let query = args[1].clone();
            let filename = args[2].clone();

            Config { query, filename }
        }
    }

Retornando tipo Result invés de chamar Panic
=============================================
.. code-block:: rust

    impl Config {
        fn new(args: &[String]) -> Result<Config, &'static str> {
            if args.len() < 3 {
                return Err("not enough arguments");
            }

            let query = args[1].clone();
            let filename = args[2].clone();

            Ok(Config { query, filename })
        }
    }

* Obs: A chamada em *main.rs* se torna um unwrap

.. code-block:: rust

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

Adaptando o erro leitura de arquivo
====================================
* lib.rs
.. code-block:: rust

    fn run(config: Config) -> Result<(), Box<dyn Error>> {
        let contents = fs::read_to_string(config.filename)?;

        println!("With text:\n{}", contents);

        Ok(())
    }

* main.rs
.. code-block:: rust

    if let Err(e) = run(config) {
        println!("Application error: {}", e);

        process::exit(1);
    }

Referenciando variável por lifetime
=====================================
.. code-block:: rust

    pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
        vec![]
    }

- Desta forma os dados do resultado serão válidos enquanto os dados em *contents* forem válidos.
- Evitando também do compilador assumir associação com *query* invés de *contents*.

Lendo variáveis de ambiente
============================
.. code-block:: rust

    let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

* Obs: foi usado *is_err* para verificação, retornando *true* se não estiver setada.

.. code-block:: bash

    $export CASE_INSENSITIVE=1