export interface EditConfig {
    max_player: number
    port: number
    password: string
    word_size: number
    word_name: string
    difficulty: number
    message: string
}

export interface GetEditConfResp {
    game_status: boolean
    config: EditConfig
}