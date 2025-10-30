package main

import (
	"fmt"
	"time"
)

func main() {
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
		retriesLeft := uint(5)

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

				if retriesLeft > 0 {
					fmt.Println("[🔃] Next retry in 5 seconds...")
					time.Sleep(5 * time.Second)
					retriesLeft -= 1
				} else {
					fmt.Printf("[❌] Skipping email %s\n", task.Email)
					break
				}

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
