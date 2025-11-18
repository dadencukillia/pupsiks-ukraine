type onStartHandler = (timer: Timer, perfomanceTime: number) => void;
type onEndHandler = (timer: Timer, perfomanceTime: number) => void;

const DEFAULT_HANDLER = (_a: Timer, _b: number) => {};

export class Timer {

  private _endPerfomanceTime = -1;
  private _busy = false;
  private _remainSeconds = $state(0);
  private _timerInterval: number|undefined = undefined;
  
  private _startHandler: onStartHandler = DEFAULT_HANDLER;
  private _endHandler: onEndHandler = DEFAULT_HANDLER;

  private updateRemainSeconds() {
    this._remainSeconds = Math.ceil(Math.max(this._endPerfomanceTime - performance.now(), 0) / 1000);
  }

  public constructor() {
    this.resetHandlers();
  }

  public runTimestampSeconds(timestampSeconds: number) {
    this.runTimestampMilli(timestampSeconds * 1_000);
  }

  public runTimestampMilli(timestampMilli: number) {
    this.runMilli(timestampMilli - Date.now());
  }

  public runSeconds(seconds: number) {
    this.runMilli(seconds * 1_000);
  }

  public runMilli(milliseconds: number) {
    if (this._busy) {
      throw new Error("AlreadyBuzyError: The timer is already in use.");
    }

    this._busy = true;
    this._endPerfomanceTime = performance.now() + milliseconds;
    this.updateRemainSeconds();
    this._startHandler(this, this._endPerfomanceTime);

    if (this.remainSeconds > 0) {
      this._timerInterval = setInterval(() => {
        this.updateRemainSeconds();

        if (this._remainSeconds <= 0) {
          this.stop();
        }
      }, 1000);
    } else {
      this.stop();
    }
  }

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

  public get remainSeconds() {
    return this._remainSeconds;
  }

  public resetEndHandler() {
    this._endHandler = DEFAULT_HANDLER;
  }

  public resetStartHandler() {
    this._startHandler = DEFAULT_HANDLER;
  }

  public resetHandlers() {
    this.resetStartHandler();
    this.resetEndHandler();
  }

  public set onStart(startHandler: onStartHandler) {
    this._startHandler = startHandler;
  }

  public set onEnd(endHandler: onEndHandler) {
    this._endHandler = endHandler;
  }
}
