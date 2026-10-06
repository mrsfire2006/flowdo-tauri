export class ServerResult<T = undefined> {
	private constructor(
		private readonly isSuccess: boolean,
		private readonly value?: T,
		private readonly errorMessage?: string,
		private readonly statusCode: number = 200
	) {}

	public get IsSuccess(): boolean {
		return this.isSuccess;
	}

	public get IsFailure(): boolean {
		return !this.isSuccess;
	}

	public get Value(): T | undefined {
		return this.value;
	}

	public get ErrorMessage(): string | undefined {
		return this.errorMessage;
	}

	public static Success<T>(value?: T): ServerResult<T> {
		return new ServerResult(true, value);
	}

	public static Failure(errorMsg: string, statusCode: number = 400): ServerResult {
		return new ServerResult(false, undefined, errorMsg, statusCode);
	}

	public GetClientResult(): ClientResult<T> {
		return {
			isSuccess: this.isSuccess,
			errorMsg: this.errorMessage,
			value: this.value,
			statusCode: this.statusCode
		};
	}
}
export interface ClientResult<T = undefined> {
	isSuccess: boolean;
	errorMsg?: string;
	value?: T;
	statusCode: number;
}
