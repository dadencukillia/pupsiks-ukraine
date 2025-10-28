package main

import (
	"fmt"
	"time"
)

func main() {
	err := SendMail("dadencukillia@gmail.com", "forgot_cert", map[string]string{
		"CERTURL": "https://google.com",
	})
	if err != nil {
		fmt.Printf("%s\n", err)
		return
	}
	fmt.Println("[✅] Successfuly sended to dadencukillia@gmail.com")

	conn := ConnectRedis()
	taskChannel := make(chan EmailTask)

	workers := GetWorkersCount()
	for range workers {
		go worker(taskChannel)
	}

	for {
		emailTask, err := GetQueueEmailTask(conn)
		if err != nil {
			continue
		}

		if emailTask != nil {
			taskChannel <- *emailTask
		}
	}
}

func worker(tasks chan EmailTask) {
	for task := range tasks {
		for {
			templateName := TemplateNameByPurpose(task.Purpose)
			if templateName == "" {
				fmt.Printf("[❌] Invalid purpose: %s\n", task.Purpose)
				break
			}

			err := SendMail(
				task.Email,
				templateName,
				task.Replacements,
			)
			if err != nil {
				fmt.Printf("%s\n", err)
				fmt.Println("[🔃] Next retry in 5 seconds...")
				time.Sleep(5 * time.Second)

				continue
			}

			fmt.Printf("[✅] Successfuly sent to %s\n", task.Email)
			break
		}
	}
}

func TemplateNameByPurpose(purpose string) string {
	switch purpose {
	case "create":
		return "create_cert"
	case "delete":
		return "delete_cert"
	case "forgot":
		return "forgot_cert"
	default:
		return ""
	}
}
