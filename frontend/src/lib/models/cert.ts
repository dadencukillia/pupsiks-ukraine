import { API_DELETE_CERT, API_GET_CERT } from "$lib/api_variables";

class CertModel {
  private id: string;
  private name: string;
  private title: string;

  public constructor(id: string, name: string, title: string) {
    this.id = id;
    this.name = name;
    this.title = title;
  }

  public getID(): string {
    return this.id;
  }

  public getName(): string {
    return this.name;
  }

  public getTitle(): string {
    return this.title;
  }

  static fromJson(json: any): CertModel {
    if (typeof json !== "object") throw TypeError("Object required");

    return new CertModel(json.id, json.name, json.title);
  }

  static getRequest(): Promise<CertModel|string> {
    return fetch(API_GET_CERT)
      .then((response): Promise<CertModel|string> => {
        if (response.ok) {
          return response.json().then(json => CertModel.fromJson(json));
        }

        return response.text();
      })
      .catch(e => {
        console.error(e);
        return "Failed to get a cert";
      });
  }
}
