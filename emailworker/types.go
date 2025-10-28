package main

type EmailTask struct {
	Purpose string `json:"purpose"`
	Email string `json:"email"`
	Replacements map[string]string `json:"replacements"`
}

type TemplateResult struct {
	Subject string
	Body string
}

