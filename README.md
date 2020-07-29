# bonus-calculator
Calculadora de bônus para caso você tenha ficado com dúvida

## Instruções de uso

Basta passa na querystring `?q=` uma string base64 com o formulário a ser montado, usando o padrão `label,valor`.

Por exemplo, para o base64 codificado `UGVyY2VudHVhbCBCYXNlLDAuNzUKTWV0YSBkZSBWZW5kYXMsMC4zMzMKU2NvcmUgZG8gVXN1w6FyaW8sMC4zMzMKQXZhbGlhw6fDo28gZG8gR2VzdG9yLDAuMzM0`:

    Percentual Base,0.75
    Meta de Vendas,0.333
    Score do Usuário,0.333
    Avaliação do Gestor,0.334

## "Pipeline" de desenvolvimento

**Como compilar o Web Assembly:**

    cd lib
    cargo update
    ./deploy_lib.sh

**Como testar o app:**

    cd www
    npm install
    npm run serve

**Como publicar (após compilar o wasm):**

    cd www
    npm install
    npm run build
