package main

import (
	"fmt"
	"os"
	"path"
	"strings"
)

func Template(name string, replacements map[string]string) (TemplateResult, error) {
	templateContent, err := os.ReadFile("templates/" + path.Clean(name) + ".html")
	if err != nil {
		return TemplateResult{}, err
	}

	template := strings.SplitN(string(templateContent), "\n", 2)
	if len(template) != 2 {
		return TemplateResult{}, fmt.Errorf("Incorrect template")
	}

	subject := template[0]
	body := template[1]

	for key, value := range replacements {
		body = strings.ReplaceAll(body, fmt.Sprintf("=^%s^=", key), value)
	}

	return TemplateResult{
		Subject: subject,
		Body: body,
	}, nil
}
