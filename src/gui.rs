//Comando para executar o exemplo "cargo run --release --features "winit glium" --example file_navigator"

extern crate time;

use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;
use conrod;
use std::path::Path;
use std::path::PathBuf;
use find_folder;
use conrod::backend::glium::glium;
use conrod::backend::glium::glium::Surface;
use conrod::backend::winit;
use support;
use teste::Teste;
use teste::Tipos;
use cenario::Cenario;
use sessao::Sessao;
use sessao::TipoSessao;

pub struct Valores{
    teste: Teste,
    sessoes: Vec<Sessao>,
    sessoes_novas: Vec<Sessao>,

    //nome: String,
    num_vals: String,
    //executavel: String,
    executavel_caminho: Option<PathBuf>,
    executavel_nome: String,
    opcao: Option<usize>,
    opcao_export: Option<usize>,
    opcao_saidas: Vec<bool>,
    opcao_cenario: Option<usize>,
    variaveis : Vec<String>,
    num_cens: String,
    cenarios: Vec<String>,
    valores: Vec<String>,
    pagina: u8,
    selecionado: u32,
    ajuda: u8,
    pagina_anterior: u8,
}

pub struct Gui{

}

impl Gui{
    pub fn new()-> Gui{
        Gui{}
    }

    pub fn run(&self) {

        const WIDTH: u32 = 1920;
        const HEIGHT: u32 = 1080;

        // Build the window.
        let mut events_loop = glium::glutin::EventsLoop::new();
        let window = glium::glutin::WindowBuilder::new()
            .with_title("Focus")
            .with_dimensions(WIDTH, HEIGHT);
        let context = glium::glutin::ContextBuilder::new()
            .with_vsync(true)
            .with_multisampling(4);
        let display = glium::Display::new(window, context, &events_loop).unwrap();

        // Construct our `Ui`.
        let mut ui = conrod::UiBuilder::new([WIDTH as f64, HEIGHT as f64]).build();

        // A unique identifier for each widget.
        //    let ids = Ids::new(ui.widget_id_generator());
        let ids = &mut Ids::new(ui.widget_id_generator());

        // Add a `Font` to the `Ui`'s `font::Map` from file.
        let assets = find_folder::Search::KidsThenParents(3, 5).for_folder("assets").unwrap();
        let font_path = assets.join("fonts/NotoSans/NotoSans-Regular.ttf");
        ui.fonts.insert_from_file(font_path).unwrap();

        // A type used for converting `conrod::render::Primitives` into `Command`s that can be used
        // for drawing to the glium `Surface`.
        let mut renderer = conrod::backend::glium::Renderer::new(&display).unwrap();

        // The image map describing each of our widget->image mappings (in our case, none).
        let image_map = conrod::image::Map::<glium::texture::Texture2d>::new();

        // Some starting text to edit.
        let mut valores = Valores{
            teste: Teste{
                nome: "Nome".to_string(),
                executavel: "Executavel".to_string(),
                variaveis: Vec::new(),
                tipo: Tipos::Vazio,
                sessao: 0,
            },
            sessoes: Vec::new(),
            sessoes_novas: Vec::new(),

            //nome: "Nome".to_string(),
            opcao: None,
            opcao_export: None,
            opcao_saidas: Vec::new(),
            opcao_cenario: None,
            valores: Vec::new(),
            variaveis: Vec::new(),
            num_vals: "0".to_string(),
            num_cens: "0".to_string(),
            cenarios: Vec::new(),
            executavel_nome: "Executavel".to_string(),
            executavel_caminho: None,
            pagina: 1,
            selecionado: 9999,
            ajuda: 0,
            pagina_anterior: 0,
        };

        // Poll events from the window.
        let mut event_loop = support::EventLoop::new();
        'main: loop {

            // Handle all events.
            for event in event_loop.next(&mut events_loop) {

                // Use the `winit` backend feature to convert the winit event to a conrod one.
                if let Some(event) = winit::convert_event(event.clone(), &display) {
                    ui.handle_event(event);
                    event_loop.needs_update();
                }

                match event {
                    glium::glutin::Event::WindowEvent { event, .. } => match event {
                        // Break from the loop upon `Escape`.
                        glium::glutin::WindowEvent::Closed |
                        glium::glutin::WindowEvent::KeyboardInput {
                            input: glium::glutin::KeyboardInput {
                                virtual_keycode: Some(glium::glutin::VirtualKeyCode::Escape),
                                ..
                            },
                            ..
                        } => break 'main,
                        _ => (),
                    },
                    _ => (),
                }
            }

            // Instnatiate all widgets in the GUI.
            if valores.pagina == 3 {
                set_widgets_3(ui.set_widgets(), ids, &mut valores);
            } else if valores.pagina == 2 {
                set_widgets_2(ui.set_widgets(), ids, &mut valores);
            } else if valores.pagina == 1{
                set_widgets_1(ui.set_widgets(), ids, &mut valores);
            } else if valores.pagina == 4 {
                set_widgets_4(ui.set_widgets(), ids, &mut valores);
            } else if valores.pagina == 5 {
                set_widgets_5(ui.set_widgets(), ids, &mut valores);
            } else if valores.pagina == 6 {
                set_widgets_6(ui.set_widgets(), ids, &mut valores);
            }else if valores.pagina == 7 {
                set_widgets_7(ui.set_widgets(), ids, &mut valores);
            }else if valores.pagina == 8 {
                set_widgets_8(ui.set_widgets(), ids, &mut valores);
            }

            // Render the `Ui` and then display it on the screen.
            if let Some(primitives) = ui.draw_if_changed() {
                renderer.fill(&display, primitives, &image_map);
                let mut target = display.draw();
                target.clear_color(0.0, 0.0, 0.0, 1.0);
                renderer.draw(&display, &mut target, &image_map).unwrap();
                target.finish().unwrap();
            }
        }
        
        //valores.sessoes = valores.teste.ler_sessoes();
        let mut arquivo = valores.teste.escrever();
        
        //valores.sessoes.append(&mut valores.sessoes_novas);

        for cada_sessao in valores.sessoes.iter(){
            cada_sessao.escrever(&mut arquivo);
        } 
        

        // Pagina de criacao ou abertura de teste.
        fn set_widgets_1(ref mut ui: conrod::UiCell, ids: &mut Ids, valores: &mut Valores) {
            use conrod::{color, widget, Colorable, Labelable, Positionable, Sizeable, Widget};
            
            widget::Canvas::new().flow_down(&[
                (ids.header, widget::Canvas::new().color(color::DARK_CHARCOAL).pad_bottom(1.0)),
                (ids.body, widget::Canvas::new().flow_right(&[
                    (ids.left_column, widget::Canvas::new().color(color::DARK_CHARCOAL)),
                    (ids.right_column, widget::Canvas::new().color(color::DARK_CHARCOAL)),
                ])),
            ]).set(ids.master, ui);

            widget::Text::new("Abrir teste ou criar novo?")
                .color(color::WHITE)
                .font_size(48)
                .middle_of(ids.header)
                .set(ids.titulo, ui);

            for _click in widget::Button::new()
                .color(color::WHITE)
                .label("Abrir")
                .label_color(color::BLACK)
                .w_h(60.0, 60.0)
                .middle_of(ids.left_column)
                .set(ids.abrir, ui)
            {
                valores.pagina = 4;
            }

            for _click in widget::Button::new()
                .color(color::WHITE)
                .label("Criar")
                .label_color(color::BLACK)
                .w_h(60.0, 60.0)
                .middle_of(ids.right_column)
                .set(ids.criar, ui)
            {
                valores.pagina = 3;
            }

            for _click in widget::Button::new()
                .color(color::WHITE)
                .label("?")
                .label_color(color::BLACK)
                .w_h(30.0, 30.0)
                .bottom_left_with_margins_on(ids.left_column, 10.0, 10.0)
                .set(ids.ajuda, ui)
            {
                valores.pagina_anterior = 1;
                valores.pagina = 7;
            }

        }

        // Pagina de abertura de executavel.
        fn set_widgets_2(ref mut ui: conrod::UiCell, ids: &mut Ids, valores: &mut Valores) {
            use conrod::{widget, Colorable, Labelable, Positionable, Sizeable, Widget};

            //let ui = &mut ui.set_widgets();
            let directory = find_folder::Search::KidsThenParents(3, 5).for_folder("Jogos").unwrap();

            widget::Canvas::new().color(conrod::color::DARK_CHARCOAL).set(ids.canvas, ui);

            for event in widget::FileNavigator::all(&directory)
                .color(conrod::color::LIGHT_BLUE)
                .font_size(16)
                .wh_of(ids.canvas)
                .middle_of(ids.canvas)
                //.show_hidden_files(true)  // Use this to show hidden files
                .set(ids.navegador_executavel, ui)
            {
                if let conrod::widget::file_navigator::Event::ChangeSelection(arquivos) = event{

                    if arquivos.len() == 1 {
                        if arquivos[0].is_file(){
                            let mut pai = arquivos[0].parent().unwrap();
                            while pai.file_name().unwrap() != Path::new("Jogos") {
                                pai = pai.parent().unwrap();
                            }

                            valores.teste.executavel = format!("{}", arquivos[0].strip_prefix(pai).unwrap().to_str().unwrap());
                            valores.executavel_nome = format!("{}", arquivos[0].file_name().unwrap().to_str().unwrap());
                            valores.executavel_caminho = Some(arquivos[0].clone());
                        }
                        
                    }
                }
                
            }

            for _click in widget::Button::new()
                .bottom_right_with_margins_on(ids.canvas, 50.0, 50.0)
                .color(conrod::color::WHITE)
                .label("Pronto")
                .label_color(conrod::color::BLACK)
                .w_h(100.0, 40.0)
                .set(ids.pronto, ui)
            {
                valores.pagina = 3;
            }

            for _click in widget::Button::new()
                .color(conrod::color::WHITE)
                .label("?")
                .label_color(conrod::color::BLACK)
                .w_h(30.0, 30.0)
                .bottom_left_with_margins_on(ids.canvas, 10.0, 10.0)
                .set(ids.ajuda, ui)
            {
                valores.pagina_anterior = 2;
                valores.pagina = 7;
            }

            for _click in widget::Button::new()
                .bottom_left_with_margins_on(ids.canvas, 10.0, 50.0)
                .color(conrod::color::WHITE)
                .label("Voltar")
                .label_color(conrod::color::BLACK)
                .w_h(100.0, 40.0)
                .set(ids.voltar, ui)
            {
                valores.pagina = 3;

            }

        }

        // Pagina de criacao de teste.
        fn set_widgets_3(ref mut ui: conrod::UiCell, ids: &mut Ids, valores: &mut Valores) {
            use conrod::{color, widget, Colorable, Labelable, Positionable, Sizeable, Widget, Borderable};

            widget::Canvas::new().flow_down(&[
                (ids.header, widget::Canvas::new().length(240.0).color(color::DARK_CHARCOAL).pad_bottom(20.0).flow_right(&[
                    (ids.left_column, widget::Canvas::new().color(color::DARK_CHARCOAL).length(500.0)),
                    (ids.right_column, widget::Canvas::new().color(color::DARK_CHARCOAL).scroll_kids_vertically().length(ui.win_w-500.0)),
                ])),
                (ids.entrada, widget::Canvas::new().color(color::WHITE).scroll_kids()),
                (ids.footer, widget::Canvas::new().length(60.0).color(color::BLACK)),
            ]).set(ids.master, ui);
            
            widget::Text::new("Criar Teste")
                .top_left_with_margins_on(ids.left_column, 20.0, 20.0)
                .color(color::WHITE)
                .font_size(48)
                .set(ids.titulo, ui);

            for event in widget::TextBox::new(&valores.teste.nome)
                .top_left_with_margins_on(ids.left_column, 80.0, 20.0)
                .font_size(20)
                .w_h(320.0, 40.0)
                .border(1.0)
                .border_color(color::BLACK)
                .color(color::WHITE)
                .set(ids.nome_edit, ui)
            {
                    match event {
                    //widget::text_box::Event::Enter => println!("TextBox: {:?}", valores.nome),
                    //widget::text_box::Event::Update(string) => valores.nome = string.to_string(),
                    widget::text_box::Event::Enter => (),
                    widget::text_box::Event::Update(string) => valores.teste.nome = string.to_string(),
                }
            }

            for event in widget::TextBox::new(&valores.executavel_nome)
                .top_left_with_margins_on(ids.left_column, 130.0, 20.0)
                .font_size(20)
                .w_h(320.0, 40.0)
                .border(1.0)
                .border_color(color::BLACK)
                .color(color::WHITE)
                .set(ids.executavel_edit, ui)
            {
                    match event {
                    //widget::text_box::Event::Enter => println!("TextBox: {:?}", valores.nome),
                    //widget::text_box::Event::Update(string) => valores.nome = string.to_string(),
                    widget::text_box::Event::Enter => (),
                    widget::text_box::Event::Update(string) => valores.teste.executavel = string.to_string(),
                }
            }

            for _click in widget::Button::new()
                .top_left_with_margins_on(ids.left_column, 130.0, 360.0)
                .color(color::WHITE)
                .label("Procurar...")
                .label_color(color::BLACK)
                .w_h(100.0, 40.0)
                .set(ids.procurar, ui)
            {
                valores.pagina = 2;
            }

            for _click in widget::Button::new()
                .color(color::WHITE)
                .label("?")
                .label_color(color::BLACK)
                .w_h(30.0, 30.0)
                .bottom_left_with_margins_on(ids.footer, 10.0, 10.0)
                .set(ids.ajuda, ui)
            {
                valores.pagina_anterior = 3;
                valores.pagina = 7;
            }

            for _click in widget::Button::new()
                .top_left_with_margins_on(ids.footer, 10.0, 50.0)
                .color(conrod::color::WHITE)
                .label("Voltar")
                .label_color(conrod::color::BLACK)
                .w_h(100.0, 40.0)
                .set(ids.voltar, ui)
            {
                valores.pagina = 1;

            }

            if valores.executavel_caminho.is_some(){
                let mut caminho_arquivo = valores.executavel_caminho.clone().unwrap();
                caminho_arquivo.set_file_name("README");
                let retorno_arquivo = File::open(caminho_arquivo);
            
                //println!("{:?}", retorno_arquivo);
                if retorno_arquivo.is_ok(){

                    let arquivo = retorno_arquivo.unwrap();
                    let mut buf_leitor = BufReader::new(arquivo);
                    let mut readme = String::new();

                    buf_leitor.read_to_string(&mut readme).unwrap();

                    widget::Text::new(&readme)
                    .top_left_with_margins_on(ids.right_column, 10.0, 20.0 )
                    .color(color::WHITE)
                    .left_justify()
                    .line_spacing(10.0)
                    .padded_w_of(ids.right_column, 20.0)
                    .wrap_by_word()
                    .set(ids.readme, ui);
                }
            }

            let tipos = vec!["Vazio".to_string(),
                                "Leve".to_string(),
                                "Cenarios".to_string()];
            

            for selecionado in widget::DropDownList::new(&tipos, valores.opcao)
                .w_h(150.0, 40.0)
                .top_left_with_margins_on(ids.left_column, 180.0, 20.0)
                .max_visible_items(3)
                .color(color::WHITE)
                .border(1.0)
                .border_color(color::BLACK)
                .label("Tipo")
                .label_color(color::BLACK)
                .scrollbar_on_top()
                .set(ids.tipos, ui)
            {
                valores.opcao = Some(selecionado);
                //println!("Escolhido: {:?}", valores.opcao);
                if selecionado == 0{
                    valores.teste.tipo = Tipos::Vazio;
                } else if selecionado == 1{
                    valores.teste.tipo = Tipos::Leve;
                } else if selecionado == 2{
                    valores.teste.tipo = Tipos::Cenarios(Vec::new());
                }
            }

            let mut num_cenarios: usize = 0;
            let mut num_variaveis: usize = 0;
            

            if valores.opcao != None && valores.opcao != Some(0) {

                widget::Text::new("Variaveis:")
                        .top_left_with_margins_on(ids.left_column, 190.0, 180.0)
                        .color(color::WHITE)
                        .font_size(20)
                        .set(ids.texto_variaveis, ui);

                for event in widget::TextBox::new(&valores.num_vals)
                    .top_left_with_margins_on(ids.left_column, 180.0, 280.0)
                    .font_size(20)
                    .w_h(30.0, 40.0)
                    .border(1.0)
                    .border_color(color::BLACK)
                    .color(color::WHITE)
                    .set(ids.num_vals, ui)
                {
                    match event {
                        //widget::text_box::Event::Enter => println!("Num vals: {:?}", valores.num_vals),
                        widget::text_box::Event::Enter => (),
                        widget::text_box::Event::Update(string) => valores.num_vals = string,
                    }
                }

                num_variaveis = match valores.num_vals.trim_right().parse::<usize>() {
                    Ok(valor) => valor,
                    Err(_) => 0 as usize,
                };

                valores.variaveis.resize(num_variaveis, "Variavel".to_string());
                
                ids.vars.resize(num_variaveis, &mut ui.widget_id_generator());

                for x in 0..num_variaveis {
                    for event in widget::TextBox::new(&valores.variaveis[x])
                        .top_left_with_margins_on(ids.entrada, 60.0 + 40.0*(x as f64), 20.0 )
                        .font_size(20)
                        .w_h(160.0, 40.0)
                        .border(1.0)
                        .border_color(color::BLACK)
                        .color(color::WHITE)
                        .set(ids.vars[x], ui)
                    {
                        match event {
                            //widget::text_box::Event::Enter => println!("TextBox {}: {:?}", x, valores.variaveis[x]),
                            widget::text_box::Event::Enter => (),
                            widget::text_box::Event::Update(string) => valores.variaveis[x] = string.to_string(),
                        }
                    }
                }

                if valores.opcao == Some(2) {

                    widget::Text::new("Cenarios:")
                        .top_left_with_margins_on(ids.left_column, 190.0, 320.0)
                        .color(color::WHITE)
                        .font_size(20)
                        .set(ids.texto_cenario, ui);

                    for event in widget::TextBox::new(&valores.num_cens)
                        .top_left_with_margins_on(ids.left_column, 180.0, 420.0)
                        .font_size(20)
                        .w_h(30.0, 40.0)
                        .border(1.0)
                        .border_color(color::BLACK)
                        .color(color::WHITE)
                        .set(ids.num_cens, ui)
                    {
                        match event {
                            //widget::text_box::Event::Enter => println!("Num cens: {:?}", valores.num_cens),
                            widget::text_box::Event::Enter => (),
                            widget::text_box::Event::Update(string) => valores.num_cens = string,
                        }
                    }
                    

                    num_cenarios = match valores.num_cens.trim_right().parse::<usize>() {
                        Ok(valor) => valor,
                        Err(_) => 0 as usize,
                    };

                    valores.cenarios.resize(num_cenarios, "Cenario".to_string());
                    
                    ids.cens.resize(num_cenarios, &mut ui.widget_id_generator());
        
                    for x in 0..num_cenarios {
                        for event in widget::TextBox::new(&valores.cenarios[x])
                            .top_left_with_margins_on(ids.entrada, 20.0, 180.0 + 100.0*(x as f64) )
                            .font_size(20)
                            .w_h(100.0, 40.0)
                            .border(1.0)
                            .border_color(color::BLACK)
                            .color(color::WHITE)
                            .set(ids.cens[x], ui)
                        {
                            match event {
                                //widget::text_box::Event::Enter => println!("TextBox {}: {:?}", x, valores.cenarios[x]),
                                widget::text_box::Event::Enter => (),
                                widget::text_box::Event::Update(string) => valores.cenarios[x] = string.to_string(),
                            }
                        }
                    }

                    let total_variaveis = num_cenarios*num_variaveis;

                    valores.valores.resize(total_variaveis, "0".to_string());
                    ids.vals.resize(total_variaveis, &mut ui.widget_id_generator());

                    for x in 0..total_variaveis{
                        for event in widget::TextBox::new(&valores.valores[x])
                            .top_left_with_margins_on(ids.entrada, 60.0 + 40.0*((x/num_cenarios) as f64), 180.0 + 100.0*((x%num_cenarios) as f64) )
                            .font_size(20)
                            .w_h(100.0, 40.0)
                            .border(1.0)
                            .border_color(color::BLACK)
                            .color(color::WHITE)
                            .set(ids.vals[x], ui)
                        {
                            match event {
                                //widget::text_box::Event::Enter => println!("TextBox {}: {:?}", x, valores.valores[x]),
                                widget::text_box::Event::Enter => (),
                                widget::text_box::Event::Update(string) => valores.valores[x] = string.to_string(),
                            }
                        }

                    }

                }

            }

            for _click in widget::Button::new()
                .top_right_with_margins_on(ids.footer, 10.0, 10.0)
                .color(color::WHITE)
                .label("Finalizar")
                .label_color(color::BLACK)
                .w_h(100.0, 40.0)
                .set(ids.ok, ui)
            {
                for x in 0..num_variaveis {
                    valores.teste.variaveis.push(valores.variaveis[x].clone());
                }

                for x in 0..num_cenarios {
                    if let Tipos::Cenarios(ref mut cenarios) = valores.teste.tipo{
                        let (valores1,valores2) = valores.valores.split_at(x*num_variaveis);
                        cenarios.push(Cenario::new(&valores.cenarios[x]).fill(&valores.teste.variaveis, valores2));
                    }
                }
                let mut arquivo = valores.teste.escrever();
                valores.pagina = 5;
            }

        }

        // Pagina de selecao de arquivo de teste.
        fn set_widgets_4(ref mut ui: conrod::UiCell, ids: &mut Ids, valores: &mut Valores) {
            use conrod::{widget, Colorable, Labelable, Positionable, Sizeable, Widget};

            //let ui = &mut ui.set_widgets();
            let directory = find_folder::Search::KidsThenParents(3, 5).for_folder("Testes").unwrap();

            widget::Canvas::new().color(conrod::color::DARK_CHARCOAL).set(ids.canvas, ui);

            for event in widget::FileNavigator::with_extension(&directory, &["kans"])
                .color(conrod::color::LIGHT_BLUE)
                .font_size(16)
                .wh_of(ids.canvas)
                .middle_of(ids.canvas)
                //.show_hidden_files(true)  // Use this to show hidden files
                .set(ids.file_navigator, ui)
            {
                if let conrod::widget::file_navigator::Event::ChangeSelection(arquivos) = event{
                    if arquivos.len() == 1 {
                        valores.teste = Teste::carregar(arquivos[0].file_stem().unwrap().to_str().unwrap(), arquivos[0].to_str().unwrap());
                        //valores.nome =format!("{}", arquivos[0].file_stem().unwrap().to_str().unwrap());
                    }
                }
            }

            for _click in widget::Button::new()
                .color(conrod::color::WHITE)
                .label("?")
                .label_color(conrod::color::BLACK)
                .w_h(30.0, 30.0)
                .bottom_left_with_margins_on(ids.canvas, 10.0, 10.0)
                .set(ids.ajuda, ui)
            {
                valores.pagina_anterior = 4;
                valores.pagina = 7;
            }

            for _click in widget::Button::new()
                .bottom_left_with_margins_on(ids.canvas, 10.0, 50.0)
                .color(conrod::color::WHITE)
                .label("Voltar")
                .label_color(conrod::color::BLACK)
                .w_h(100.0, 40.0)
                .set(ids.voltar, ui)
            {
                valores.pagina = 1;

            }

            for _click in widget::Button::new()
                .bottom_right_with_margins_on(ids.canvas, 50.0, 50.0)
                .color(conrod::color::WHITE)
                .label("Pronto")
                .label_color(conrod::color::BLACK)
                .w_h(100.0, 40.0)
                .set(ids.pronto, ui)
            {
                match valores.teste.tipo{
                    Tipos::Vazio => valores.opcao = Some(0),
                    Tipos::Leve => {
                        valores.opcao = Some(1);
                        valores.variaveis.clear();
                        valores.num_vals = valores.teste.variaveis.len().to_string();
                        for i in 0..valores.teste.variaveis.len(){
                            valores.variaveis.push(valores.teste.variaveis[i].clone());
                        }
                    },
                    Tipos::Cenarios(ref cenarios) => {
                        valores.opcao = Some(2);
                        valores.num_vals = valores.teste.variaveis.len().to_string();
                        valores.num_cens = cenarios.len().to_string();
                    },
                };
                valores.sessoes = valores.teste.ler_sessoes();
                valores.fill();
                valores.pagina = 5;
                

            }

        }

        // Pagina de teste aberto.
        fn set_widgets_5(ref mut ui: conrod::UiCell, ids: &mut Ids, valores: &mut Valores) {
            use conrod::{color, widget, Colorable, Labelable, Positionable, Sizeable, Widget, Borderable};

            widget::Canvas::new().flow_down(&[
                (ids.header, widget::Canvas::new().length(200.0).color(color::DARK_CHARCOAL).pad_bottom(20.0)),
                (ids.entrada, widget::Canvas::new().color(color::WHITE).scroll_kids()),
                (ids.footer, widget::Canvas::new().length(60.0).color(color::BLACK)),
            ]).set(ids.master, ui);

            for _click in widget::Button::new()
                .top_right_with_margins_on(ids.footer, 10.0, 10.0)
                .color(color::WHITE)
                .label("Executar")
                .label_color(color::BLACK)
                .w_h(100.0, 40.0)
                .set(ids.executar, ui)
            {
                    valores.pagina = 8;
            }

            for _click in widget::Button::new()
                .top_right_with_margins_on(ids.footer, 10.0, 120.0)
                .color(color::WHITE)
                .label("Exportar")
                .label_color(color::BLACK)
                .w_h(100.0, 40.0)
                .set(ids.exportar, ui)
            {
                    valores.pagina = 6;
            }

            for _click in widget::Button::new()
                .top_right_with_margins_on(ids.footer, 10.0, 240.0)
                .color(color::WHITE)
                .label("Processar")
                .label_color(color::BLACK)
                .w_h(100.0, 40.0)
                .set(ids.processar, ui)
            {
                for cada_sessao in valores.sessoes.iter_mut(){
                    cada_sessao.processar_video();
                }    
                valores.pagina = 5;
            }

            for _click in widget::Button::new()
                .color(color::WHITE)
                .label("?")
                .label_color(color::BLACK)
                .w_h(30.0, 30.0)
                .bottom_left_with_margins_on(ids.footer, 10.0, 10.0)
                .set(ids.ajuda, ui)
            {
                valores.pagina_anterior = 5;
                valores.pagina = 7;
            }

            /*for _click in widget::Button::new()
                .top_right_with_margins_on(ids.footer, 10.0, 360.0)
                .color(color::WHITE)
                .label("Editar")
                .label_color(color::BLACK)
                .w_h(100.0, 40.0)
                .set(ids.editar, ui)
            {
                    valores.pagina = 3;
            }*/

            widget::Text::new("Nome do Teste: ")
                .top_left_with_margins_on(ids.header, 20.0, 20.0)
                .color(color::WHITE)
                .font_size(48)
                .set(ids.title_aux, ui);

            widget::Text::new(valores.teste.nome.as_str())
                .top_left_with_margins_on(ids.header, 20.0, 370.0)
                .color(color::RED)
                .font_size(48)
                .set(ids.title, ui);

            widget::Text::new("Nome do Executável: ")
                .top_left_with_margins_on(ids.header, 80.0, 20.0)
                .color(color::WHITE)
                .font_size(48)
                .set(ids.executavel_aux, ui);

            widget::Text::new(valores.teste.executavel.as_str())
                .top_left_with_margins_on(ids.header, 80.0, 490.0)
                .color(color::RED)
                .font_size(48)
                .set(ids.executavel, ui);

            widget::Text::new("Tipo do Teste: ")
                .top_left_with_margins_on(ids.header, 140.0, 20.0)
                .color(color::WHITE)
                .font_size(24)
                .set(ids.tipo_aux, ui);

            let tipos = vec!["Vazio".to_string(),
                                "Leve".to_string(),
                                "Cenarios".to_string()];

            widget::Text::new(tipos[valores.opcao.unwrap()].as_str())
                .top_left_with_margins_on(ids.header, 140.0, 180.0)
                .color(color::RED)
                .font_size(24)
                .set(ids.tipo, ui);            

            
            if valores.opcao != None && valores.opcao != Some(0) {

                let num_variaveis = match valores.num_vals.trim_right().parse::<usize>() {
                    Ok(valor) => valor,
                    Err(_) => 0 as usize,
                };

                ids.vars_nome.resize(num_variaveis, &mut ui.widget_id_generator());

                for x in 0..num_variaveis {
                    widget::Text::new(valores.variaveis[x].as_str())
                        .top_left_with_margins_on(ids.entrada, 60.0 + 40.0*(x as f64), 20.0 )
                        .font_size(20)
                        .w_h(160.0, 40.0)
                        .color(color::BLACK)
                        .set(ids.vars_nome[x], ui);
                
                }

                if valores.opcao == Some(1){

                    valores.valores.resize(num_variaveis, "0".to_string());
                    ids.vars.resize(num_variaveis, &mut ui.widget_id_generator());

                    for x in 0..num_variaveis {
                        for event in widget::TextBox::new(&valores.valores[x])
                            .top_left_with_margins_on(ids.entrada, 55.0 + 40.0*(x as f64), 120.0 )
                            .font_size(20)
                            .w_h(160.0, 40.0)
                            .border(1.0)
                            .border_color(color::BLACK)
                            .color(color::WHITE)
                            .center_justify()
                            .set(ids.vars[x], ui)
                        {
                            match event {
                                //widget::text_box::Event::Enter => println!("TextBox {}: {:?}", x, valores.variaveis[x]),
                                widget::text_box::Event::Enter => (),
                                widget::text_box::Event::Update(string) => valores.valores[x] = string.to_string(),
                            }
                        }
                    }
                }else{

                    let mut color_saidas = color::WHITE;

                    if let None = valores.opcao_cenario {
                        color_saidas = color::DARK_GREEN;
                    }

                    for _click in widget::Button::new()
                            .top_left_with_margins_on(ids.entrada, 20.0, 20.0 )
                            .color(color_saidas)
                            .label("Aleatorio")
                            .label_color(color::BLACK)
                            .w_h(160.0, 40.0)
                            .set(ids.aleatorio, ui)
                        {

                                valores.opcao_cenario = None;
                        }

                    let num_cenarios = match valores.num_cens.trim_right().parse::<usize>() {
                        Ok(valor) => valor,
                        Err(_) => 0 as usize,
                    };

                    ids.cens_nome.resize(num_cenarios, &mut ui.widget_id_generator());

                    for x in 0..num_cenarios {

                        color_saidas = color::WHITE;
                        if let Some(escolha) = valores.opcao_cenario {
                            if escolha == x{
                                color_saidas = color::DARK_GREEN;
                            }
                        }
                        for _click in widget::Button::new()
                            .top_left_with_margins_on(ids.entrada, 20.0, 180.0 + 100.0*(x as f64) )
                            .color(color_saidas)
                            .label(valores.cenarios[x].as_str())
                            .label_color(color::BLACK)
                            .w_h(100.0, 40.0)
                            .set(ids.cens_nome[x], ui)
                        {

                                valores.opcao_cenario = Some(x);
                        }
                    }

                    /*
                        widget::Text::new(valores.cenarios[x].as_str())
                            .top_left_with_margins_on(ids.entrada, 20.0, 180.0 + 100.0*(x as f64) )
                            .font_size(20)
                            .w_h(100.0, 40.0)
                            .color(color::BLACK)
                            .set(ids.cens_nome[x], ui);
                    }
                    */
                    let total_variaveis = num_cenarios*num_variaveis;

                    ids.vals_nome.resize(total_variaveis, &mut ui.widget_id_generator());

                    for x in 0..total_variaveis{
                        widget::Text::new(valores.valores[x].as_str())
                            .top_left_with_margins_on(ids.entrada, 60.0 + 40.0*((x/num_cenarios) as f64), 180.0 + 100.0*((x%num_cenarios) as f64) )
                            .font_size(20)
                            .w_h(100.0, 40.0)
                            .color(color::BLACK)
                            .center_justify()
                            .set(ids.vals_nome[x], ui);
                    }
                }
            }

            for _click in widget::Button::new()
                .top_left_with_margins_on(ids.footer, 10.0, 50.0)
                .color(conrod::color::WHITE)
                .label("Voltar")
                .label_color(conrod::color::BLACK)
                .w_h(100.0, 40.0)
                .set(ids.voltar, ui)
            {
                //valores.zerar();
                valores.pagina = 1;

            }
        }

        //Pagina de exportar teste.
        fn set_widgets_6(ref mut ui: conrod::UiCell, ids: &mut Ids, valores: &mut Valores) {
            use conrod::{color, widget, Colorable, Labelable, Positionable, Sizeable, Widget, Borderable};

            widget::Canvas::new().flow_down(&[
                (ids.header, widget::Canvas::new().length(130.0).color(color::DARK_CHARCOAL).pad_bottom(20.0)),
                (ids.entrada, widget::Canvas::new().color(color::WHITE).scroll_kids().flow_right(&[
                    (ids.left_column, widget::Canvas::new().color(color::WHITE).scroll_kids().length(150.0)),
                    (ids.right_column, widget::Canvas::new().color(color::WHITE).scroll_kids()),
                ])),
                (ids.footer, widget::Canvas::new().length(60.0).color(color::BLACK)),
            ]).set(ids.master, ui);

            let mut selecionado = 0;
            let mut labels: Vec<String> = Vec::new();
            let mut labels_temp: Vec<String> = Vec::new();
            let mut selecionados = 0;

            if valores.opcao_export == Some(1) {

                 //println!("{:?}", valores.sessoes);
                if valores.sessoes.len() > 0
                {
                    if valores.selecionado != 9999 {selecionado = valores.selecionado as usize;}
                    labels_temp = valores.sessoes[selecionado].dados.nomes_variaveis();


                    if labels_temp.len() != valores.opcao_saidas.len(){
                        valores.opcao_saidas.clear();
                        for x in 0..labels_temp.len()
                        {
                            valores.opcao_saidas.push(false);
                        }
                    }

                    ids.selecionar_saidas.resize(valores.opcao_saidas.len() as usize, &mut ui.widget_id_generator());
                    ids.rectangle_saida.resize(valores.opcao_saidas.len() as usize, &mut ui.widget_id_generator());

                    for x in 0..labels_temp.len(){
                            let mut color_saidas = color::WHITE;
                        if valores.opcao_saidas[x] {
                            color_saidas = color::DARK_GREEN;
                        }
                        for _click in widget::Button::new()
                            .top_left_with_margins_on(ids.left_column, 50.0 + 40.0*((x) as f64), 10.0)
                            .color(color_saidas)
                            .label(&labels_temp[x])
                            .label_color(color::BLACK)
                            .w_h(100.0, 40.0)
                            .set(ids.selecionar_saidas[x as usize], ui)
                        {

                                valores.opcao_saidas[x] = !valores.opcao_saidas[x];
                        }
                    }
                }

                for x in 0..valores.opcao_saidas.len()
                {
                    if valores.opcao_saidas[x]
                    {
                        selecionados += 1;
                        labels.push(labels_temp[x].clone());
                    }
                }
                
                let numero_var = format!("{}/8",selecionados);
                widget::Text::new(&numero_var)
                    .top_left_with_margins_on(ids.left_column, 10.0, 10.0 )
                    .font_size(20)
                    .color(color::BLACK)
                    .set(ids.numero, ui);
               
            }

            for _click in widget::Button::new()
                .top_right_with_margins_on(ids.footer, 10.0, 10.0)
                .color(color::WHITE)
                .label("Exportar")
                .label_color(color::BLACK)
                .w_h(100.0, 40.0)
                .set(ids.exportar, ui)
            {
                valores.pagina = 5;
                

                if valores.opcao_export == Some(0){
                // Exportar como CSV

                    if valores.sessoes.len() > 0{
                        let mut arquivo = valores.teste.escrever();
        
                        for cada_sessao in valores.sessoes.iter_mut(){
                            cada_sessao.processar_video();
                            cada_sessao.escrever(&mut arquivo);
                        }
                        valores.teste.exportar(&valores.sessoes);  
                    }
                } else if valores.opcao_export == Some(1){
                    //Exportar como gráfico

                    let mut arquivo = valores.teste.escrever();
        
                    for cada_sessao in valores.sessoes.iter_mut(){
                        cada_sessao.processar_video();
                        cada_sessao.escrever(&mut arquivo);
                    }

                    valores.sessoes[valores.selecionado as usize].exportar_grafico(selecionados, labels.clone());

                } else if valores.opcao_export == Some(2){
                    //Exportar como vídeo

                    valores.sessoes[valores.selecionado as usize].exportar_video();

                    let mut arquivo = valores.teste.escrever();

                    for cada_sessao in valores.sessoes.iter_mut(){
                        cada_sessao.processar_video();
                        cada_sessao.escrever(&mut arquivo);
                    }
                }

            }

            for _click in widget::Button::new()
                .top_left_with_margins_on(ids.footer, 10.0, 50.0)
                .color(color::WHITE)
                .label("Voltar")
                .label_color(color::BLACK)
                .w_h(100.0, 40.0)
                .set(ids.voltar, ui)
            {
                valores.pagina = 5;

            }

            for _click in widget::Button::new()
                .color(color::WHITE)
                .label("?")
                .label_color(color::BLACK)
                .w_h(30.0, 30.0)
                .bottom_left_with_margins_on(ids.footer, 10.0, 10.0)
                .set(ids.ajuda, ui)
            {
                valores.pagina_anterior = 6;
                valores.pagina = 7;
            }

            widget::Text::new("Exportar como:")
                .top_left_with_margins_on(ids.header, 10.0, 10.0 )
                .font_size(48)
                .color(color::WHITE)
                .set(ids.titulo, ui);

            let tipos = vec!["CSV".to_string(),
                            "Gráfico".to_string(),
                            "Vídeo".to_string()];

            for selecionado in widget::DropDownList::new(&tipos, valores.opcao_export)
                .w_h(150.0, 40.0)
                .top_left_with_margins_on(ids.header, 80.0, 20.0)
                .max_visible_items(3)
                .color(color::WHITE)
                .border(1.0)
                .border_color(color::BLACK)
                .label("Opções")
                .label_color(color::BLACK)
                .scrollbar_on_top()
                .set(ids.tipos, ui)
            {
                valores.opcao_export = Some(selecionado);
            }            

            if valores.opcao_export == Some(1) || valores.opcao_export == Some(2){
                widget::Text::new("Sessao")
                    .top_left_with_margins_on(ids.right_column, 10.0 , 120.0 )
                    .font_size(20)
                    .w_h(100.0, 40.0)
                    .color(color::BLACK)
                    .set(ids.xis, ui);

                widget::Text::new("Inicio")
                    .top_left_with_margins_on(ids.right_column, 10.0 , 120.0 + 110.0 )
                    .font_size(20)
                    .w_h(200.0, 40.0)
                    .color(color::BLACK)
                    .set(ids.data, ui);

                widget::Text::new("Conclusão")
                    .top_left_with_margins_on(ids.right_column, 10.0 , 120.0 + 320.0 )
                    .font_size(20)
                    .w_h(200.0, 40.0)
                    .color(color::BLACK)
                    .set(ids.ypslom, ui);
                            
                            

                ids.sessoes.resize((valores.sessoes.len()*3) as usize, &mut ui.widget_id_generator());
                ids.rectangle.resize((valores.sessoes.len()) as usize, &mut ui.widget_id_generator());
                ids.selecionar.resize((valores.sessoes.len()) as usize, &mut ui.widget_id_generator());

                for x in 0..(valores.sessoes.len() as u32){

                    if x == valores.selecionado {
                        widget::Rectangle::fill([640.0, 40.0])
                            .top_left_with_margins_on(ids.right_column, 50.0 + 40.0*((x) as f64), 10.0)
                            .color(color::DARK_GREEN)
                            .set(ids.rectangle[x as usize], ui);
                    } else {
                        widget::Rectangle::outline([640.0, 40.0])
                            .top_left_with_margins_on(ids.right_column, 50.0 + 40.0*((x) as f64), 10.0)
                            .color(color::DARK_GREEN)
                            .set(ids.rectangle[x as usize], ui);
                    }

                    for _click in widget::Button::new()
                        .top_left_with_margins_on(ids.right_column, 50.0 + 40.0*((x) as f64), 10.0)
                        .color(color::WHITE)
                        .label("Escolher")
                        .label_color(color::BLACK)
                        .w_h(100.0, 40.0)
                        .set(ids.selecionar[x as usize], ui)
                    {
                            valores.selecionado = x;
                            //println!("{:?}", valores.selecionado);
                    }

                    let mut diferencial = 0.0;

                    for y in 0..3{
                        let texto: String;
                        if y == 0 {
                            texto = format!("{}",valores.sessoes[x as usize].sessao_atual);
                        } else if y == 1{
                            texto = get_data(valores.sessoes[x as usize].data_inicio);
                        } else{
                            texto = get_data(valores.sessoes[x as usize].data_conclusao);
                        }
                        if y==2{
                            diferencial = 100.0;
                        }
                        widget::Text::new(texto.as_str())
                            .top_left_with_margins_on(ids.right_column, 60.0 + 40.0*((x) as f64), 120.0 + diferencial + 100.0*((y) as f64) )
                            .font_size(20)
                            .w_h(200.0, 40.0)
                            .color(color::BLACK)
                            .set(ids.sessoes[(x*3 + y) as usize], ui);
                    }
                }
            }
            
        }

        //Pagina de mostra de ajuda
        fn set_widgets_7(ref mut ui: conrod::UiCell, ids: &mut Ids, valores: &mut Valores) {
            use conrod::{color, widget, Colorable, Labelable, Positionable, Sizeable, Widget};
            
            let nome_arquivo = format!("./Ajuda/ajuda_{}.txt", valores.pagina_anterior);
            let arquivo = File::open(nome_arquivo)
                .expect("Erro ao abrir arquivo de teste.");
            let mut buf_leitor = BufReader::new(arquivo);
            let mut ajuda = String::new();

            buf_leitor.read_to_string(&mut ajuda).unwrap();

            widget::Canvas::new().flow_down(&[
                (ids.header, widget::Canvas::new().color(color::DARK_CHARCOAL).pad_bottom(1.0).length(100.0)),
                (ids.body, widget::Canvas::new().color(color::DARK_CHARCOAL)),
                (ids.footer, widget::Canvas::new().color(color::DARK_CHARCOAL).length(100.0)),
            ]).set(ids.master, ui);

            widget::Text::new("Ajuda")
                .color(color::WHITE)
                .font_size(48)
                .top_left_with_margins_on(ids.header, 20.0, 20.0)
                .set(ids.titulo, ui);

            for _click in widget::Button::new()
                .color(color::WHITE)
                .label("Fechar")
                .label_color(color::BLACK)
                .w_h(60.0, 60.0)
                .bottom_right_with_margins_on(ids.footer, 20.0, 20.0)
                .set(ids.fechar, ui)
            {
                valores.pagina = valores.pagina_anterior;
            }
            

            widget::Text::new(&ajuda)
                .top_left_with_margins_on(ids.body, 20.0, 20.0)
                .color(color::WHITE)
                .font_size(20)
                .set(ids.texto_ajuda, ui);

        }

        //Pagina de executar jogo
        fn set_widgets_8(ref mut ui: conrod::UiCell, ids: &mut Ids, valores: &mut Valores) {
            use conrod::{color, widget, Colorable, Labelable, Positionable, Sizeable, Widget};
            
            widget::Canvas::new().color(conrod::color::DARK_CHARCOAL).set(ids.canvas, ui);

            for _click in widget::Button::new()
                .color(color::WHITE)
                .label("Iniciar Jogo")
                .label_color(color::BLACK)
                .w_h(ui.w_of(ids.canvas).unwrap()/5.0, ui.h_of(ids.canvas).unwrap()/5.0)
                .middle_of(ids.canvas)
                .set(ids.abrir, ui)
            {
                    valores.teste.sessao += 1;
                    let tipo = match valores.teste.tipo {
                        Tipos::Cenarios(_) => Some(TipoSessao::Cenario(String::new())),
                        Tipos::Leve => Some(TipoSessao::Leve(valores.valores.clone())),
                        Tipos::Vazio => None,
                    };
                    let mut sessao = Sessao::new(&valores.teste.nome, valores.teste.sessao, tipo);
                    sessao.iniciar(&valores.teste, valores.opcao_cenario);
                    valores.sessoes.push(sessao);
            }

            for _click in widget::Button::new()
                .bottom_left_with_margins_on(ids.canvas, 10.0, 50.0)
                .color(color::WHITE)
                .label("Voltar")
                .label_color(color::BLACK)
                .w_h(100.0, 40.0)
                .set(ids.voltar, ui)
            {
                valores.pagina = 5;

            }

            for _click in widget::Button::new()
                .color(color::WHITE)
                .label("?")
                .label_color(color::BLACK)
                .w_h(30.0, 30.0)
                .bottom_left_with_margins_on(ids.canvas, 10.0, 10.0)
                .set(ids.ajuda, ui)
            {
                valores.pagina_anterior = 8;
                valores.pagina = 7;
            }

        }

        pub fn get_data(data:time::Tm)-> String{
            format!("{}/{}/{}-{}:{}:{}", data.tm_year + 1900, data.tm_mon, data.tm_mday, data.tm_hour, data.tm_min, data.tm_sec)
        }

        // Generate a unique `WidgetId` for each widget.
        widget_ids! {
            struct Ids {
                canvas,
                master,
                header,
                footer,
                body,
                left_column,
                right_column,
                title,

                voltar,
                abrir,
                criar,
                navegador_executavel,
                pronto,
                procurar,
                file_navigator,
                titulo,
                nome_edit,
                executavel_edit,
                tipos,
                num_vals,
                num_cens,
                vars[],
                vars_nome[],
                cens[],
                cens_nome[],
                vals[],
                vals_nome[],
                ok,
                entrada,
                executar,
                exportar,
                processar,
                editar,
                executavel,
                tipo,
                sessoes[],
                xis,
                data,
                ypslom,
                rectangle[],
                selecionar[],
                selecionar_saidas[],
                rectangle_saida[],
                tipo_aux,
                title_aux,
                executavel_aux,
                texto_cenario,
                texto_variaveis,
                aleatorio,
                readme,
                fechar,
                ajuda,
                texto_ajuda,
                numero,
            }
        }
    }

}

impl Valores{
    pub fn fill(&mut self){

        match self.teste.tipo{
            Tipos::Vazio => (),
            Tipos::Leve => {
                self.variaveis = self.teste.variaveis.clone();
            },
            Tipos::Cenarios(ref cenarios) => {
                self.variaveis = self.teste.variaveis.clone();
                self.cenarios.clear();
                self.valores.clear();
                for cenario in cenarios{
                    self.cenarios.push(cenario.nome.clone());
                    for valor in cenario.variaveis.iter(){
                        self.valores.push(valor.as_string());
                    }
                }
            },
        }

    }

    pub fn zerar(&mut self){
        self.teste = Teste{
                nome: "Nome".to_string(),
                executavel: "Executavel".to_string(),
                variaveis: Vec::new(),
                tipo: Tipos::Vazio,
                sessao: 0,
            };
        self.sessoes = Vec::new();
        self.sessoes_novas = Vec::new();
        self.opcao = None;
        self.opcao_export = None;
        self.opcao_saidas = vec![false, false, false, false, false, false, false];
        self.opcao_cenario = None;
        self.valores = Vec::new();
        self.variaveis = Vec::new();
        self.num_vals = "0".to_string();
        self.num_cens = "0".to_string();
        self.cenarios = Vec::new();
        //executavel: "Executavel".to_string(),
        self.executavel_caminho = None;
        self.pagina = 1;
        self.selecionado = 9999;
        self.ajuda = 0;
        self.pagina_anterior = 0;
    }
}