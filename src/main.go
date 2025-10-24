package main

import (
	"encoding/json"
	"math/rand"
	"strings"
	"syscall/js"
	"time"
)

type Idioma string

const (
	Portugues Idioma = "pt"
	Ingles    Idioma = "en"
)

func (i Idioma) Traduzir(texto string) string {
	switch i {
	case Portugues:
		switch texto {
		case "senha_gerada":
			return "Senha Gerada"
		case "copiada":
			return "Copiada!"
		case "tamanho":
			return "Comprimento da senha"
		case "maiusculas":
			return "Letras Maiúsculas"
		case "numeros":
			return "Números"
		case "simbolos":
			return "Especiais"
		case "gerar":
			return "Gerar Nova Senha"
		case "baixar":
			return "Baixar 10 Senhas"
		default:
			return texto
		}
	case Ingles:
		switch texto {
		case "senha_gerada":
			return "Generated Password"
		case "copiada":
			return "Copied!"
		case "tamanho":
			return "Password length"
		case "maiusculas":
			return "Uppercase Letters"
		case "numeros":
			return "Numbers"
		case "simbolos":
			return "Special Characters"
		case "gerar":
			return "Generate New Password"
		case "baixar":
			return "Download 10 Passwords"
		default:
			return texto
		}
	default:
		return texto
	}
}

type Senhas struct {
	ContemNumeros    bool
	ContemSimbolos   bool
	ContemMaiusculas bool
	Idioma           Idioma
}

func NovoSenhas() *Senhas {
	return &Senhas{
		ContemNumeros:    true,
		ContemSimbolos:   true,
		ContemMaiusculas: true,
		Idioma:           Portugues,
	}
}

func (s *Senhas) Gerar(tamanho int) string {
	caracteres := "abcdefghijklmnopqrstuvwxyz"
	if s.ContemMaiusculas {
		caracteres += "ABCDEFGHIJKLMNOPQRSTUVWXYZ"
	}
	if s.ContemNumeros {
		caracteres += "0123456789"
	}
	if s.ContemSimbolos {
		caracteres += "!@#$%^&*()_+-=[]{}|;:,.<>?"
	}

	rand.Seed(time.Now().UnixNano())
	var sb strings.Builder
	for i := 0; i < tamanho; i++ {
		idx := rand.Intn(len(caracteres))
		sb.WriteByte(caracteres[idx])
	}
	return sb.String()
}

func (s *Senhas) GerarMultiplas(tamanho, quantidade int) string {
	lista := make([]string, quantidade)
	for i := 0; i < quantidade; i++ {
		lista[i] = s.Gerar(tamanho)
	}
	return strings.Join(lista, "\n")
}

func (s *Senhas) GetLabels() string {
	labels := map[string]string{
		"senha_gerada": s.Idioma.Traduzir("senha_gerada"),
		"copiada":      s.Idioma.Traduzir("copiada"),
		"tamanho":      s.Idioma.Traduzir("tamanho"),
		"maiusculas":   s.Idioma.Traduzir("maiusculas"),
		"numeros":      s.Idioma.Traduzir("numeros"),
		"simbolos":     s.Idioma.Traduzir("simbolos"),
		"gerar":        s.Idioma.Traduzir("gerar"),
		"baixar":       s.Idioma.Traduzir("baixar"),
	}
	b, _ := json.Marshal(labels)
	return string(b)
}

func GeneratePassword(length int, useUppercase, useNumbers, useSymbols bool) string {
	caracteres := "abcdefghijklmnopqrstuvwxyz"
	if useUppercase {
		caracteres += "ABCDEFGHIJKLMNOPQRSTUVWXYZ"
	}
	if useNumbers {
		caracteres += "0123456789"
	}
	if useSymbols {
		caracteres += "!@#$%^&*()_+-=[]{}|;:,.<>?"
	}

	rand.Seed(time.Now().UnixNano())
	var sb strings.Builder
	for i := 0; i < length; i++ {
		idx := rand.Intn(len(caracteres))
		sb.WriteByte(caracteres[idx])
	}
	return sb.String()
}

// Armazenar instâncias usando mapa global
var senhasMap = map[int]*Senhas{}
var nextID = 0

func newSenhas(this js.Value, args []js.Value) interface{} {
	s := NovoSenhas()
	id := nextID
	nextID++
	senhasMap[id] = s
	return js.ValueOf(id)
}

func getInstance(args []js.Value) *Senhas {
	id := args[0].Int()
	return senhasMap[id]
}

func setNumeros(this js.Value, args []js.Value) interface{} {
	s := getInstance(args)
	s.ContemNumeros = args[1].Bool()
	return nil
}

func setSimbolos(this js.Value, args []js.Value) interface{} {
	s := getInstance(args)
	s.ContemSimbolos = args[1].Bool()
	return nil
}

func setMaiusculas(this js.Value, args []js.Value) interface{} {
	s := getInstance(args)
	s.ContemMaiusculas = args[1].Bool()
	return nil
}

func setIdioma(this js.Value, args []js.Value) interface{} {
	s := getInstance(args)
	if args[1].String() == "en" {
		s.Idioma = Ingles
	} else {
		s.Idioma = Portugues
	}
	return nil
}

func gerar(this js.Value, args []js.Value) interface{} {
	s := getInstance(args)
	tamanho := args[1].Int()
	return s.Gerar(tamanho)
}

func gerarMultiplas(this js.Value, args []js.Value) interface{} {
	s := getInstance(args)
	tamanho := args[1].Int()
	quantidade := args[2].Int()
	return s.GerarMultiplas(tamanho, quantidade)
}

func getLabels(this js.Value, args []js.Value) interface{} {
	s := getInstance(args)
	return s.GetLabels()
}

func generatePasswordWrapper(this js.Value, args []js.Value) interface{} {
	length := args[0].Int()
	useUppercase := args[1].Bool()
	useNumbers := args[2].Bool()
	useSymbols := args[3].Bool()
	return GeneratePassword(length, useUppercase, useNumbers, useSymbols)
}

func main() {
	c := make(chan struct{}, 0)

	js.Global().Set("newSenhas", js.FuncOf(newSenhas))
	js.Global().Set("setNumeros", js.FuncOf(setNumeros))
	js.Global().Set("setSimbolos", js.FuncOf(setSimbolos))
	js.Global().Set("setMaiusculas", js.FuncOf(setMaiusculas))
	js.Global().Set("setIdioma", js.FuncOf(setIdioma))
	js.Global().Set("gerar", js.FuncOf(gerar))
	js.Global().Set("gerarMultiplas", js.FuncOf(gerarMultiplas))
	js.Global().Set("getLabels", js.FuncOf(getLabels))
	js.Global().Set("generatePassword", js.FuncOf(generatePasswordWrapper))

	<-c
}
