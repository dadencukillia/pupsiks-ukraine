import os
import json
import smtplib
from email.mime.text import MIMEText
from email.header import Header
from redis import Redis, exceptions as redis_exc
from dotenv import load_dotenv
import time


# Configurations
load_dotenv()

REDIS_HOST = "redis"
REDIS_PORT = 6379
EMAIL_QUEUE = os.getenv("EMAIL_QUEUE", "email_jobs")
QUEUE_TIMEOUT = 10

SMTP_SERVER = os.getenv("SMTP_SERVER", "smtp.example.com")
SMTP_PORT = int(os.getenv("SMTP_PORT", 587))
SMTP_USERNAME = os.getenv("SMTP_USERNAME", "noreply@example.com")
SMTP_PASSWORD = os.getenv("SMTP_PASSWORD", "your_secure_password")

MAX_RETRIES = 5
RETRY_DELAY = 30


# Functions
def send_email_via_smtp(to_email: str, subject: str, body: str):
    print(f"Sending email: {to_email}")
    
    msg = MIMEText(body, 'plain', 'utf-8')
    msg['Subject'] = Header(subject, 'utf-8')
    msg['From'] = SMTP_USERNAME
    msg['To'] = to_email

    try:
        # Використовуємо TLS для безпечного з'єднання (Port 587)
        with smtplib.SMTP(SMTP_SERVER, SMTP_PORT) as server:
            server.starttls() 
            server.login(SMTP_USERNAME, SMTP_PASSWORD)
            server.sendmail(SMTP_USERNAME, [to_email], msg.as_string())
        
        print(f"Sent email: {to_email}")
        return True

    except smtplib.SMTPException as e:
        print(f"SMTP Sending error to {to_email}: {e}")
        return False
    except Exception as e:
        print(f"Unknown sending error to {to_email}: {e}")
        return False


def process_message(message_data: str, redis_client: Redis):
    try:
        job = json.loads(message_data)
        to_email = job.get('to_email')
        subject = job.get('subject')
        body = job.get('body')
        retries = job.get('retries', 0)

        if not all([to_email, subject, body]):
            print(f"Invalid message_data: {job}")
            return
        
        success = send_email_via_smtp(to_email, subject, body)

        if not success:
            if retries < MAX_RETRIES:
                job['retries'] = retries + 1
                
                print(f"Sending error. Retries: #{job['retries']} in {RETRY_DELAY} seconds. Mail: {to_email}")
                
                time.sleep(RETRY_DELAY) 
                
                redis_client.rpush(EMAIL_QUEUE, json.dumps(job))
            else:
                print(f"No retries left. Mail: {to_email}")


    except json.JSONDecodeError:
        print(f"Invalid message_data: {message_data}")
    except Exception as e:
        print(f"Unknown error: {e}")


def worker_loop():
    try:
        print("Connecting to redis...")
        redis_client = Redis(host=REDIS_HOST, port=REDIS_PORT, decode_responses=True)
        redis_client.ping()
        print("Email worker was started!")
        print(f"Email worker is listening {EMAIL_QUEUE}")

        while True:
            item = redis_client.brpop(EMAIL_QUEUE, timeout=QUEUE_TIMEOUT)

            if item:
                queue_name, message_data = item
                process_message(message_data, redis_client)
            
    except redis_exc.ConnectionError as e:
        print(f"Redis connection error: {e}. Restarting...")
        time.sleep(3)
        worker_loop()
    except KeyboardInterrupt:
        print("\nInterrupted")
        exit(0)


def main():
    worker_loop()

if __name__ == "__main__":
    main()
