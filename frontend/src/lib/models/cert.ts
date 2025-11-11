class CertModel {
  private id: string;
  private name: string;
  private email: string;
  private title: string;

  public constructor(id: string, name: string, email: string, title: string) {
    this.id = id;
    this.name = name;
    this.email = email;
    this.title = title;
  }

  public getID(): string {
    return this.id;
  }

  public getName(): string {
    return this.name;
  }

  public getEmail(): string {
    return this.email;
  }

  public getTitle(): string {
    return this.title;
  }

  static fromJson(json: any): CertModel {
    if (typeof json !== "object") throw TypeError("Object is required");

    return new CertModel(json.id??"", json.name??"", json.email??"", json.title??"");
  }

  toJson(): string {
    return JSON.stringify(this);
  }
}
