package main

import (
	"os"
	"runtime"
	"strconv"
)

func GetRedisAddress() string {
	return "redis:6379"
}

func GetRedisQueueKey() string {
	return "email_jobs"
}

func GetWorkersCount() uint {
	workersStr := os.Getenv("WORKERS")
	if workers, err := strconv.Atoi(workersStr); err != nil {
		if workers > 0 {
			return uint(workers)
		}
	}

	return uint(runtime.NumCPU())
}

func GetSmtpServer() string {
	return os.Getenv("SMTP_SERVER")
}

func GetSmtpPort() uint {
	portStr := os.Getenv("SMTP_PORT")
	port, err := strconv.Atoi(portStr)
	if err != nil {
		return 587
	}

	return uint(port)
}

func GetSmtpSender() string {
	return os.Getenv("SMTP_SENDER")
}

func GetSmtpUsername() string {
	return os.Getenv("SMTP_USERNAME")
}

func GetSmtpPassword() string {
	return os.Getenv("SMTP_PASSWORD")
}
