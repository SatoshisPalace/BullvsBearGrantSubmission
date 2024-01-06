export type ContestInfo = {
    id: number;
    options: ContestOutcome[];  // Assuming ContestOutcome can be represented, adjust as necessary
    timeOfClose: number;
    timeOfResolve: number;
    eventDetails: string;
};

export type UserContest = {
    address: string;  // Assuming Addr can be represented as string
    contestId: number;
};

export type ContestOutcome = {
    id: number;  // u8 in Rust is a small integer, so it's represented as number in TypeScript
    name: string;
};
