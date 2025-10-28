package main

import (
	"fmt"
	gomail "gopkg.in/mail.v2"
)

func SendMail(recipient string, template string, replacements map[string]string) error {
	smtpHost := GetSmtpServer()
	smtpPort := GetSmtpPort()
	sender := GetSmtpSender()
	username := GetSmtpUsername()
	password := GetSmtpPassword()

	templateResult, err := Template(template, replacements)
	if err != nil {
		return fmt.Errorf("[❌] No template was found: %s", err)
	}

	m := gomail.NewMessage()
	m.SetHeader("From", sender)
	m.SetHeader("To", recipient)
	m.SetHeader("Subject", templateResult.Subject)
	m.SetBody("text/html", templateResult.Body)

	d := gomail.NewDialer(smtpHost, int(smtpPort), username, password)
	if err := d.DialAndSend(m); err != nil {
		return fmt.Errorf("[❌] Error sending email to %s: %s", recipient, err)
	}

	return nil
}
