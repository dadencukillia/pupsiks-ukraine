/**
 * Type definition for the handler function executed when the timer starts.
 */
type onStartHandler = (timer: Timer, perfomanceTime: number) => void;
/**
 * Type definition for the handler function executed when the timer ends or is stopped.
 */
type onEndHandler = (timer: Timer, perfomanceTime: number) => void;

const DEFAULT_HANDLER = (_a: Timer, _b: number) => {};

/**
 * A utility class for managing a countdown timer using high-resolution time (`performance.now()`).
 *
 * It provides methods to run the timer based on duration or specific timestamps.
 */
export class Timer {
  private _endPerfomanceTime = -1;
  private _busy = false;
  private _remainSeconds = $state(0); // Reactive magic
  private _timerInterval: number|undefined = undefined;
  
  private _startHandler: onStartHandler = DEFAULT_HANDLER;
  private _endHandler: onEndHandler = DEFAULT_HANDLER;

  /**
   * Recalculates the remaining seconds based on the current high-resolution time
   * and the stored end time (`_endPerfomanceTime`).
   */
  private _updateRemainSeconds() {
    this._remainSeconds = Math.ceil(Math.max(this._endPerfomanceTime - performance.now(), 0) / 1000);
  }

  /**
   * Initializes a new instance of the Timer class.
   */
  public constructor() {
    this.resetHandlers();
  }

  /**
   * Runs the timer until a specific UNIX timestamp (in seconds).
   *
   * @param timestampSeconds - The target UNIX timestamp in seconds.
   * @throws {Error} If the timer is already busy.
   */
  public runTimestampSeconds(timestampSeconds: number) {
    this.runTimestampMillis(timestampSeconds * 1_000);
  }

  /**
   * Runs the timer until a specific UNIX timestamp (in milliseconds).
   *
   * @param timestampMillis - The target UNIX timestamp in milliseconds.
   * @throws {Error} If the timer is already busy.
   */
  public runTimestampMillis(timestampMillis: number) {
    this.runMillis(timestampMillis - Date.now());
  }

  /**
   * Runs the timer for a specified duration in seconds.
   *
   * @param seconds - The duration in seconds.
   * @throws {Error} If the timer is already busy.
   */
  public runSeconds(seconds: number) {
    this.runMillis(seconds * 1_000);
  }

  /**
   * Starts the countdown timer for the specified duration in milliseconds.
   *
   * @param milliseconds - The duration for the timer in milliseconds.
   * @throws {Error} If the timer is already busy.
   */
  public runMillis(milliseconds: number) {
    if (this._busy) {
      throw new Error("AlreadyBuzyError: The timer is already in use.");
    }

    this._busy = true;
    this._endPerfomanceTime = performance.now() + milliseconds;
    this._updateRemainSeconds();
    this._startHandler(this, this._endPerfomanceTime);

    if (this.remainSeconds > 0) {
      this._timerInterval = setInterval(() => {
        this._updateRemainSeconds();

        if (this._remainSeconds <= 0) {
          this.stop();
        }
      }, 1000);
    } else {
      this.stop();
    }
  }

  /**
   * Stops the timer, clears the interval, and optionally executes the `onEnd` handler.
   *
   * @param notifyHandlers - If true (default), the `onEnd` handler will be called.
   */
  public stop(notifyHandlers = true) {
    if (!this._busy) {
      return;
    }

    clearInterval(this._timerInterval);
    if (notifyHandlers) {
      this._endHandler(this, this._endPerfomanceTime);
    }
    this._remainSeconds = 0;
    this._endPerfomanceTime = -1;
    this.resetEndHandler();
    this._busy = false;
  }

  /**
   * Returns the current number of seconds remaining in the countdown.
   */
  public get remainSeconds() {
    return this._remainSeconds;
  }

  /**
   * Resets the `onEnd` handler to the default no-op function.
   */
  public resetEndHandler() {
    this._endHandler = DEFAULT_HANDLER;
  }

  /**
   * Resets the `onStart` handler to the default no-op function.
   */
  public resetStartHandler() {
    this._startHandler = DEFAULT_HANDLER;
  }

  /**
   * Resets both `onStart` and `onEnd` handlers to the default no-op function.
   */
  public resetHandlers() {
    this.resetStartHandler();
    this.resetEndHandler();
  }

  /**
   * Sets the handler function to be executed when the timer starts.
   */
  public set onStart(startHandler: onStartHandler) {
    this._startHandler = startHandler;
  }

  /**
   * Sets the handler function to be executed when the timer ends or is stopped.
   */
  public set onEnd(endHandler: onEndHandler) {
    this._endHandler = endHandler;
  }
}
