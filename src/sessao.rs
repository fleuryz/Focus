extern crate time;
extern crate nix;
extern crate rand;
extern crate scrap;

use dados::TipoDados;
use dados::Dado;
use variavel::Variavel;
use teste::Tipos;
use teste::Teste;

use std::path::Path;
use std::path::PathBuf;

use std::process::Command;
use std;
use std::fs::File;
use nix::sys::signal::Signal;
use nix::unistd::Pid;
use std::thread;
use rand::Rng;
use std::io;
use std::sync::mpsc;

use std::io::Write;

use os_type;

#[derive(Debug)]
pub enum TipoSessao {
    Cenario(String),
    Leve(Vec<String>),
}

#[derive(Debug)]
pub struct Sessao {
    nome_teste: String,
    pub sessao_atual: u32,
    pub data_inicio: time::Tm,
    pub data_conclusao: time::Tm,
    pub arquivo_video: String,
    pub dados: TipoDados,
    tipo: Option<TipoSessao>,
}

impl Sessao{
    pub fn new(nome_teste: &str, sessao_atual: u32, tipo: Option<TipoSessao>) -> Sessao{
        Sessao{
            nome_teste: String::from(nome_teste),
            sessao_atual,
            data_inicio: time::empty_tm(),
            data_conclusao: time::empty_tm(),
            arquivo_video: format!("{}_{}.avi",nome_teste, sessao_atual),
            dados: TipoDados::DadosBrutos( Vec::new() ),
             tipo,
        }
    }

    pub fn iniciar(&mut self, teste: &Teste, escolha_cenario: Option<usize>){

        let (tx,rx) = mpsc::channel();
        let (tx2, rx2) = mpsc::channel();
        let (tx3, rx3) = mpsc::channel();

        let executavel = format!("./{}", &teste.executavel);

        let argumentos:Vec<String> = match teste.tipo{
            Tipos::Vazio => {
                self.tipo = None;
                Vec::new()
            },
            Tipos::Leve => {
                let mut retorno:Vec<String> = Vec::new();
                if let Some(TipoSessao::Leve(ref valores_variaveis)) = self.tipo{
                    retorno = valores_variaveis.clone();
                }else{
                    retorno = Vec::new();
                }
                retorno
            },
            Tipos::Cenarios(ref cenarios) => {
                let mut arg_temp:Vec<String> = Vec::new();
                let num_cenario = match escolha_cenario{
                    Some(valor) => valor,
                    None => rand::thread_rng().gen_range(0, cenarios.len()),
                };
                let mut valor_variavel;
                let nome_cenario = cenarios[num_cenario].nome.clone();
                for variavel in cenarios[num_cenario].variaveis.iter() {
                    valor_variavel = match variavel {
                        &Variavel::Int(ref valor) => format!("{}",valor),
                        &Variavel::Float(ref valor) => format!("{}",valor),
                        &Variavel::Booleano(ref valor) => format!("{}",valor),
                        &Variavel::Texto(ref valor) => valor.clone(),
                    };
                    arg_temp.push(valor_variavel);
                }
                self.tipo = Some(TipoSessao::Cenario(nome_cenario));
                arg_temp
            },
        };

        let thread1 = thread::spawn(move || {

            let mensagem_erro = format!("Erro ao executar o jogo: ({}).",PathBuf::from(executavel.clone()).file_name().unwrap().to_str().unwrap());
            let executavel_temp = PathBuf::from(executavel.clone());

            let pasta = format!("./Jogos/{}/", executavel_temp.parent().unwrap().strip_prefix("./").unwrap().to_str().unwrap());
            
            let output = Command::new( format!("./{}",PathBuf::from(executavel).file_name().unwrap().to_str().unwrap()))
                .args(argumentos)
                .current_dir(pasta)
                .output()
                .expect(&mensagem_erro);
            
            for linha in output.stdout {
                tx.send(linha).unwrap();
            }
            //assert!(output.status.success());   
            
        });

        let mut nome_video:String = format!("./Videos/{}",self.arquivo_video);
        
        self.data_inicio = time::now();
        let thread2 = thread::spawn(move || {

            let mut filmagem: std::process::Child;

            match os_type::current_platform().os_type {
                os_type::OSType::OSX => {
                      filmagem = Command::new("ffmpeg")
                        .args(&["-video_size", "640x480", "-framerate", "30", "-f", "avfoundation", "-i", "0:none", "-preset", "veryfast", &nome_video])
                        .spawn()
                        .expect("Camera was not recorded");
                }
                os_type::OSType::Ubuntu => {
                    filmagem = Command::new("./Python/salvarCam.py")
                        .arg(nome_video)
                        .spawn()
                        .expect("Camera was not recorded");
                }
                _ => {
                    println!("I can't tell what system this is.");
                    filmagem = Command::new("ls")
                        .spawn()
                        .expect("Erro");
                }
            }
            
            loop{
                if rx2.try_recv().is_ok() {
                    nix::sys::signal::kill(Pid::from_raw(filmagem.id() as i32), Signal::SIGINT).unwrap();
                    //filmagem.kill().expect("Não foi possivel encerrar filmagem.");
                    return;
                }
            }

        });

        let nome_video:String = format!("./Videos/tela_{}",self.arquivo_video);

        let thread3 = thread::spawn(move || {

            let mut filmagem: std::process::Child;

            match os_type::current_platform().os_type {
                os_type::OSType::OSX => {
                      filmagem = Command::new("ffmpeg")
                        .args(&["-f", "avfoundation","-r", "30", "-i", "1:0", "-vf", "crop=1920:1080:0:0", "-preset", "veryfast", &nome_video])
                        .spawn()
                        .expect("Screen was not recorded");
                }
                os_type::OSType::Ubuntu => {
                    filmagem = Command::new("ffmpeg")
                        .args(&["-video_size", "1920x1080", "-framerate", "30", "-f", "x11grab", "-i", ":0.0", "-f", "alsa", "-ac", "2", "-i", "hw:0", 
                            "-c:v", "libx264", "-crf", "0", "-preset", "ultrafast", &nome_video])
                        .spawn()
                        .expect("Screen was not recorded");
                }
                _ => {
                    println!("I can't tell what system this is.");
                    filmagem = Command::new("ls")
                        .spawn()
                        .expect("Erro");
                }
            }
        
            

            
            loop{
                if rx3.try_recv().is_ok() {
                    nix::sys::signal::kill(Pid::from_raw(filmagem.id() as i32), Signal::SIGINT).unwrap();
                    //filmagem.kill().expect("Não foi possivel encerrar filmagem.");
                    break;
                }
            }

        });

        thread1.join().unwrap(); 
    
        tx2.send("fim").unwrap();
        
        thread2.join().unwrap();

        tx3.send("fim").unwrap();
        
        thread3.join().unwrap();

        self.data_conclusao = time::now();

        let mut info: Vec<u8> = Vec::new();

        for mensagens in rx {
            info.push(mensagens);
        } 

        for linha in String::from_utf8(info).unwrap().lines(){   
            if let TipoDados::DadosBrutos(ref mut dados_brutos) = self.dados{
                dados_brutos.push(Dado::get_dados(linha));
            }
        }
    }

    pub fn iniciar_texto(&mut self, teste: &Teste){

        let (tx,rx) = mpsc::channel();
        let (tx2, rx2) = mpsc::channel();
        let (tx3, rx3) = mpsc::channel();

        let executavel = format!("./{}", &teste.executavel);

        let argumentos:Vec<String> = match teste.tipo{
            Tipos::Vazio => {
                self.tipo = None;
                Vec::new()
            },
            Tipos::Leve => {
                let mut arg_temp:Vec<String> = Vec::new();
                let mut valores_variaveis:Vec<String> = Vec::new(); 
                for variavel in teste.variaveis.iter(){
                    let mut valor_receb = String::new();
                    println!("Qual o valor de {}?",variavel);
                    io::stdin().read_line(&mut valor_receb)
                        .expect("Erro ao receber valor de variável leve.");
                    valor_receb = String::from(valor_receb.trim());
                    arg_temp.push(valor_receb.clone());
                    valores_variaveis.push(valor_receb);
                }
                self.tipo = Some(TipoSessao::Leve(valores_variaveis));
                arg_temp
            },
            Tipos::Cenarios(ref cenarios) => {
                let mut arg_temp:Vec<String> = Vec::new();
                let num_cenario = rand::thread_rng().gen_range(0, cenarios.len());
                let mut valor_variavel;
                let nome_cenario = cenarios[num_cenario].nome.clone();
                for variavel in cenarios[num_cenario].variaveis.iter() {
                    valor_variavel = match variavel {
                        &Variavel::Int(ref valor) => format!("{}",valor),
                        &Variavel::Float(ref valor) => format!("{}",valor),
                        &Variavel::Booleano(ref valor) => format!("{}",valor),
                        &Variavel::Texto(ref valor) => valor.clone(),
                    };
                    arg_temp.push(valor_variavel);
                }
                self.tipo = Some(TipoSessao::Cenario(nome_cenario));
                arg_temp
            },
        };

        let thread1 = thread::spawn(move || {
            let output = Command::new(executavel)
                .args(argumentos)
                .current_dir("./Jogos/")
                .output()
                .expect("Erro ao executar o jogo.");
            for linha in output.stdout {
                tx.send(linha).unwrap();
            }
         
            //assert!(output.status.success());   
            
        });

        let nome_video:String = format!("{}",self.arquivo_video);
        
        self.data_inicio = time::now();
        let thread2 = thread::spawn(move || {
        
            let filmagem = Command::new("./Python/salvarCam.py")
                .arg(nome_video)
                .spawn()
                .expect("Camera was not recorded");
            
            loop{
                if rx2.try_recv().is_ok() {
                    nix::sys::signal::kill(Pid::from_raw(filmagem.id() as i32), Signal::SIGINT).unwrap();
                    //filmagem.kill().expect("Não foi possivel encerrar filmagem.");
                    return;
                }
            }

        });

        let nome_video2:String = format!("{}",self.arquivo_video);

        println!("Vai abrir a thread");

        let thread3 = thread::spawn(move || {

            println!("Comecando thread");
        
            let filmagem = Command::new("./Python/salvarTela.py")
                .arg(nome_video2)
                .spawn()
                .expect("Screen was not recorded");

            println!("Vai comecar");
            
            loop{
                if rx3.try_recv().is_ok() {
                    nix::sys::signal::kill(Pid::from_raw(filmagem.id() as i32), Signal::SIGINT).unwrap();
                    //filmagem.kill().expect("Não foi possivel encerrar filmagem.");
                    break;
                }
            }

           let output = filmagem
            .wait_with_output()
            .expect("failed to wait on child");

            println!("{:?}", output)

        });

        println!("Segue a Thread:{:?}", thread3);

        thread1.join().unwrap(); 
    
        tx2.send("fim").unwrap();
        
        thread2.join().unwrap();

        tx3.send("fim").unwrap();
        
        thread3.join().unwrap();
        
        self.data_conclusao = time::now();

        let mut info: Vec<u8> = Vec::new();

        for mensagens in rx {
            info.push(mensagens);
        } 

        for linha in String::from_utf8(info).unwrap().lines(){   
            if let TipoDados::DadosBrutos(ref mut dados_brutos) = self.dados{
                dados_brutos.push(Dado::get_dados(linha));
            }
        }
    }

    pub fn to_tm(data: &str) -> time::Tm{
        let valores:Vec<_> = data.split('/').collect();
        let mut ano = valores[0].parse().unwrap();
        ano = ano - 1900;
        time::Tm{
            tm_sec: valores[5].parse().unwrap(),
            tm_min: valores[4].parse().unwrap(),
            tm_hour: valores[3].parse().unwrap(),
            tm_mday: valores[2].parse().unwrap(),
            tm_mon: valores[1].parse().unwrap(),
            tm_year: ano,
            tm_wday: 0,
            tm_yday: 0,
            tm_isdst: 0,
            tm_utcoff: 0,
            tm_nsec: valores[6].parse().unwrap(),
        }
    }

    pub fn escrever_data(arquivo:&mut File, hora:time::Tm){
        let data = format!("{}/{}/{}/{}/{}/{}/{}", hora.tm_year + 1900, hora.tm_mon, hora.tm_mday, hora.tm_hour,
            hora.tm_min, hora.tm_sec, hora.tm_nsec);
        
        arquivo.write(data.as_bytes()).unwrap();
    }

    pub fn data_string(hora:time::Tm) -> String{
        format!("{}/{}/{}/{}/{}/{}/{}", hora.tm_year + 1900, hora.tm_mon, hora.tm_mday, hora.tm_hour,
            hora.tm_min, hora.tm_sec, hora.tm_nsec)
    }

    pub fn escrever(&self, arquivo:&mut File){
        let sessao = format!("Sessao:\n{}\n", self.sessao_atual);
        arquivo.write(sessao.as_bytes()).unwrap();
        match self.tipo {
            Some(ref tipo) => {
                match tipo {
                    &TipoSessao::Cenario(ref cenario) => {
                        let valor_cenario = format!("Cenario:\n{}\n",cenario);
                        arquivo.write(valor_cenario.as_bytes()).unwrap();
                    },
                    &TipoSessao::Leve(ref valores) => {
                        arquivo.write(b"Leve:\n").unwrap();
                        for valor in valores.iter(){
                            arquivo.write(valor.as_bytes()).unwrap();
                            arquivo.write(b"\n").unwrap();
                        }
                    },
                    
                }
            },
            None => (),
        }
        Sessao::escrever_data(arquivo, self.data_inicio);
        arquivo.write(b"\n").unwrap();
        Sessao::escrever_data(arquivo, self.data_conclusao);
        arquivo.write(b"\n").unwrap();
        arquivo.write(self.arquivo_video.as_bytes()).unwrap();
        arquivo.write(b"\n").unwrap();
        self.dados.escrever(arquivo);
        arquivo.write(b".").unwrap();
    }

    pub fn processar_video(&mut self) -> Option<()>{
        let mut novos_dados:Vec<Dado> = Vec::new();
        match self.dados {
            TipoDados::DadosProcessados(_) => return None,
            TipoDados::DadosBrutos(ref mut dados) => {
                let programa_video = "./Python/processarVideo.py" ;
                let output = Command::new(programa_video)
                    .arg(self.arquivo_video.as_str())
                    .arg(Sessao::data_string(self.data_inicio))
                    .output()
                    .expect("Erro ao executar");

                
                for linha in String::from_utf8(output.stdout).unwrap().lines(){
                    dados.push(Dado::get_dados(linha));
                }

                dados.sort();
                for dado in dados.iter() {
                    novos_dados.push(dado.copiar());
                }

		//fs::remove_file(format!("./Videos/{}", arquivo_video)).unwrap();
		
            },
        }
        self.dados = TipoDados::DadosProcessados(novos_dados);
		
        Some( () )
    }

    pub fn exportar_grafico(&self, selecionados:u32, variaveis: Vec<String>){
        match self.dados {
            TipoDados::DadosProcessados(_) => {
                let programa = "./Python/exportarGrafico.py";
                let tempo = self.data_conclusao.to_timespec() - self.data_inicio.to_timespec();
                let output = Command::new(programa)
                    .arg(self.nome_teste.as_str())
                    .arg(format!("{}", self.sessao_atual))
                    .arg(Sessao::data_string(self.data_inicio))
                    .arg(format!("{}", tempo.num_milliseconds()))
                    .arg(format!("{}", selecionados))
                    .args(&variaveis)
                    .output()
                    .expect("Erro ao executar exportacao de video");
                //println!("{:?}", output);

            },
            TipoDados::DadosBrutos(_) => {
                println!("É necessário processar a sessão antes.");
            },
        }
    }

    pub fn exportar_video(&mut self)-> Option<()> {
        let mut novos_dados:Vec<Dado> = Vec::new();
        let mut atualizar_dados = false;
        match self.dados {
            TipoDados::DadosProcessados(_) => {
                let programa_video = "./Python/exportarVideo.py";
                let output = Command::new(programa_video)
                    .arg(self.arquivo_video.as_str())
                    .arg(Sessao::data_string(self.data_inicio))
                    .arg("0")
                    .output()
                    .expect("Erro ao executar exportacao de video");
                //println!("{:?}", output);
            },
            TipoDados::DadosBrutos(ref mut dados) => {
                let programa_video = "./Python/exportarVideo.py";
                let output = Command::new(programa_video)
                    .arg(self.arquivo_video.as_str())
                    .arg(Sessao::data_string(self.data_inicio))
                    .arg("1")
                    .output()
                    .expect("Erro ao executar exportacao de video");

                for linha in String::from_utf8(output.stdout).unwrap().lines(){
                    dados.push(Dado::get_dados(linha));
                }

                dados.sort();
                
                for dado in dados.iter() {
                    novos_dados.push(dado.copiar());
                }

                atualizar_dados = true;
            },
        }

        if atualizar_dados{
            self.dados = TipoDados::DadosProcessados(novos_dados);
        }

        Some( () )
    }

}