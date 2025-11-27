package main

import (
	"context"
	"encoding/json"
	"fmt"
	"time"

	"github.com/redis/go-redis/v9"
)

func ConnectRedis() *redis.Client {
	redisAddr := GetRedisAddress()

	rdb := redis.NewClient(&redis.Options{
		Addr:     redisAddr,
		Password: "",
		DB:       0,
	})

	for {
		cmd := rdb.Ping(context.Background())
		if err := cmd.Err(); err != nil {
			fmt.Printf("[❌] Can't connect to redis (addr: %s) %s\n", redisAddr, err)
			fmt.Println("[🔃] Retry in 5 seconds...")

			time.Sleep(5 * time.Second)
			continue
		}

		fmt.Println("[✅] Works fine with redis!")
		break
	}

	return rdb
}

func GetQueueEmailTask(conn *redis.Client) (*EmailTask, error) {
	emailElementsResult := conn.BRPop(
		context.Background(),
		10*time.Second,
		GetRedisQueueKey(),
	)

	emailElements, err := emailElementsResult.Result()
	if err != nil {
		return nil, fmt.Errorf("[❌] Can't get queue element from redis: %s", err)
	}

	if len(emailElements) != 2 {
		return nil, nil
	}

	var emailTask EmailTask

	emailElement := emailElements[1]
	if err := json.Unmarshal([]byte(emailElement), &emailTask); err != nil {
		fmt.Println("[❌] Queue element is invalid")
		return nil, fmt.Errorf("[❌] Queue element is invalid")
	}

	fmt.Println(emailElement)
	return &emailTask, nil
}
