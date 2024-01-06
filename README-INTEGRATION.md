# Usage

Add the following to your `package.json` `dependencies`
```
	"Contest": "github:SatoshisPalace/SatoshisPalaceContest"
```

Add the following to your projects `tscongig.json` `compilerOptions`:
```
    "baseUrl": ".", // The base directory for resolving modules.
    "paths": {
      "Contest": ["node_modules/Contest/ts-src/index"]
    }
```

# Development

ChatGPT4 template to convert Rust Execute/Query messages into typescript interfaces:
```
I need you to convert some Rust enums into typescript interfaces for me.
For Example if I give you these Rust Enums vairants:
#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    SetViewingKey {
        key: String,
        padding: Option<String>,
    },
    Mint {
        recipient: String,
        amount: Uint128,
        memo: Option<String>,
        decoys: Option<Vec<Addr>>,
        entropy: Option<Binary>,
        padding: Option<String>,
    },
}
I would like you to convert them to these typescript interfaces and their accompanying cummulative type:
export type MintMsg = {
    mint: {
        recipient: string;
        amount: string;
        memo?: string 
        decoys?: string[]  // Assuming Vec<Addr> can be represented as array of strings
        entropy?: string  // Assuming Binary can be represented as string
        padding?: string 
    };
};
export type SetViewingKeyMsg = {
    setViewingKey: {
        key: string;
        padding?: string;
    };
};
// Combined Type for all ExecuteMsg variants
export type ExecuteMsg =
    | SetViewingKeyMsg
    | MintMsg
    ;


Note how the Pascal Case in rust is converted to snake case in typescript.
Return only the converted typescript interfaces and their cummulative type, in a single code block, and write as much code as you can, I will ask you to continue should the task go beyond your text generation limit.

Please convert the following Rust Enum:

```

## Compile
```
nvm use 18
```
```
npm install
```
```
npm run build
```
## Tests
```
npm run test
```