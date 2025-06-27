use gtk4::prelude::*;
use gtk4::{glib, Application, ApplicationWindow, EventControllerKey};

use std::path::Path;

use webkit6::{WebView};
use webkit6::prelude::WebViewExt;

const APP_ID: &str = "com.exemplo.htmlviewer";

fn main() -> glib::ExitCode {
    let app = Application::builder()
        .application_id(APP_ID)
        .build();

    app.connect_activate(build_ui);
    app.run()
}

fn build_ui(app: &Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("HTML Viewer")
        .default_width(800)
        .default_height(600)
        .build();

    let webview = WebView::new();
    
    let file_path = "index.html";
    // Tenta obter o caminho absoluto do arquivo.
    // Se o arquivo não existir ou o caminho for inválido, este expect entrará em pânico.
    let absolute_path = std::fs::canonicalize(file_path)
        .expect("Não foi possível obter o caminho absoluto de index.html. Certifique-se de que 'index.html' existe no diretório de execução.");
    
    // Tenta ler o conteúdo do arquivo HTML.
    // Se o arquivo não puder ser lido, este expect entrará em pânico.
    let html_content = std::fs::read_to_string(file_path)
        .expect("Não foi possível ler o arquivo HTML. Certifique-se de que 'index.html' tem permissões de leitura.");
    
    // Obtém o diretório pai para o URI base.
    let base_uri = format!("file://{}", absolute_path.parent().unwrap().display());
    webview.load_html(&html_content, Some(&base_uri));

    // Cria um controlador de evento para teclas
    let key_controller = EventControllerKey::new();

    // Clona a referência da janela para ser usada na closure do evento de teclado
    let window_clone = window.clone();
    key_controller.connect_key_pressed(move |_, keyval, _, _| {
        // Verifica se a tecla pressionada é F11
        // `gdk4::Key::F11` representa o código da tecla F11
        if keyval == gdk4::Key::F11 {
            // Verifica o estado atual de fullscreen da janela
            if window_clone.is_fullscreen() {
                // Se estiver em tela cheia, sai da tela cheia
                window_clone.unfullscreen();
            } else {
                // Se não estiver em tela cheia, entra em tela cheia
                window_clone.fullscreen();
            }
            // Retorna um valor booleano indicando se o evento foi tratado.
            // Retornar `glib::Propagation::Stop` impede que o evento se propague para outros manipuladores.
            glib::Propagation::Stop
        } else {
            // Se não for F11, permite que o evento se propague.
            glib::Propagation::Proceed
        }
    });

    // Adiciona o controlador de evento à janela
    window.add_controller(key_controller);

    window.set_child(Some(&webview));
    window.present();
}