use jlabel::{
    AccentPhraseCurrent, AccentPhrasePrevNext, BreathGroupCurrent, BreathGroupPrevNext, Label,
    Mora, Phoneme, Utterance, Word,
};

pub fn fixtures() -> [(&'static str, Label); 12] {
    [
        // こんにちは
        (
            "xx^xx-sil+k=o/A:xx+xx+xx/B:xx-xx_xx/C:xx_xx+xx/D:xx+xx_xx/E:xx_xx!xx_xx-xx/F:xx_xx#xx_xx@xx_xx|xx_xx/G:5_5%0_xx_xx/H:xx_xx/I:xx-xx@xx+xx&xx-xx|xx+xx/J:1_5/K:1+1-5",
            Label {
                phoneme: Phoneme {
                    p2: None,
                    p1: None,
                    c: Some("sil".to_string()),
                    n1: Some("k".to_string()),
                    n2: Some("o".to_string()),
                },
                mora: None,
                word_prev: None,
                word_curr: None,
                word_next: None,
                accent_phrase_prev: None,
                accent_phrase_curr: None,
                accent_phrase_next: Some(AccentPhrasePrevNext {
                    mora_count: 5,
                    accent_position: 5,
                    is_interrogative: false,
                    is_pause_insertion: None,
                }),
                breath_group_prev: None,
                breath_group_curr: None,
                breath_group_next: Some(BreathGroupPrevNext {
                    accent_phrase_count: 1,
                    mora_count: 5,
                }),
                utterance: Utterance {
                    breath_group_count: 1,
                    accent_phrase_count: 1,
                    mora_count: 5,
                },
            },
        ),
        (
            "xx^sil-k+o=N/A:-4+1+5/B:xx-xx_xx/C:09_xx+xx/D:xx+xx_xx/E:xx_xx!xx_xx-xx/F:5_5#0_xx@1_1|1_5/G:xx_xx%xx_xx_xx/H:xx_xx/I:1-5@1+1&1-1|1+5/J:xx_xx/K:1+1-5",
            Label {
                phoneme: Phoneme {
                    p2: None,
                    p1: Some("sil".to_string()),
                    c: Some("k".to_string()),
                    n1: Some("o".to_string()),
                    n2: Some("N".to_string()),
                },
                mora: Some(Mora {
                    relative_accent_position: -4,
                    position_forward: 1,
                    position_backward: 5,
                }),
                word_prev: None,
                word_curr: Some(Word {
                    pos: Some(9),
                    ctype: None,
                    cform: None,
                }),
                word_next: None,
                accent_phrase_prev: None,
                accent_phrase_curr: Some(AccentPhraseCurrent {
                    mora_count: 5,
                    accent_position: 5,
                    is_interrogative: false,
                    accent_phrase_position_forward: 1,
                    accent_phrase_position_backward: 1,
                    mora_position_forward: 1,
                    mora_position_backward: 5,
                }),
                accent_phrase_next: None,
                breath_group_prev: None,
                breath_group_curr: Some(BreathGroupCurrent {
                    accent_phrase_count: 1,
                    mora_count: 5,
                    breath_group_position_forward: 1,
                    breath_group_position_backward: 1,
                    accent_phrase_position_forward: 1,
                    accent_phrase_position_backward: 1,
                    mora_position_forward: 1,
                    mora_position_backward: 5,
                }),
                breath_group_next: None,
                utterance: Utterance {
                    breath_group_count: 1,
                    accent_phrase_count: 1,
                    mora_count: 5,
                },
            },
        ),
        (
            "sil^k-o+N=n/A:-4+1+5/B:xx-xx_xx/C:09_xx+xx/D:xx+xx_xx/E:xx_xx!xx_xx-xx/F:5_5#0_xx@1_1|1_5/G:xx_xx%xx_xx_xx/H:xx_xx/I:1-5@1+1&1-1|1+5/J:xx_xx/K:1+1-5",
            Label {
                phoneme: Phoneme {
                    p2: Some("sil".to_string()),
                    p1: Some("k".to_string()),
                    c: Some("o".to_string()),
                    n1: Some("N".to_string()),
                    n2: Some("n".to_string()),
                },
                mora: Some(Mora {
                    relative_accent_position: -4,
                    position_forward: 1,
                    position_backward: 5,
                }),
                word_prev: None,
                word_curr: Some(Word {
                    pos: Some(9),
                    ctype: None,
                    cform: None,
                }),
                word_next: None,
                accent_phrase_prev: None,
                accent_phrase_curr: Some(AccentPhraseCurrent {
                    mora_count: 5,
                    accent_position: 5,
                    is_interrogative: false,
                    accent_phrase_position_forward: 1,
                    accent_phrase_position_backward: 1,
                    mora_position_forward: 1,
                    mora_position_backward: 5,
                }),
                accent_phrase_next: None,
                breath_group_prev: None,
                breath_group_curr: Some(BreathGroupCurrent {
                    accent_phrase_count: 1,
                    mora_count: 5,
                    breath_group_position_forward: 1,
                    breath_group_position_backward: 1,
                    accent_phrase_position_forward: 1,
                    accent_phrase_position_backward: 1,
                    mora_position_forward: 1,
                    mora_position_backward: 5,
                }),
                breath_group_next: None,
                utterance: Utterance {
                    breath_group_count: 1,
                    accent_phrase_count: 1,
                    mora_count: 5,
                },
            },
        ),
        (
            "k^o-N+n=i/A:-3+2+4/B:xx-xx_xx/C:09_xx+xx/D:xx+xx_xx/E:xx_xx!xx_xx-xx/F:5_5#0_xx@1_1|1_5/G:xx_xx%xx_xx_xx/H:xx_xx/I:1-5@1+1&1-1|1+5/J:xx_xx/K:1+1-5",
            Label {
                phoneme: Phoneme {
                    p2: Some("k".to_string()),
                    p1: Some("o".to_string()),
                    c: Some("N".to_string()),
                    n1: Some("n".to_string()),
                    n2: Some("i".to_string()),
                },
                mora: Some(Mora {
                    relative_accent_position: -3,
                    position_forward: 2,
                    position_backward: 4,
                }),
                word_prev: None,
                word_curr: Some(Word {
                    pos: Some(9),
                    ctype: None,
                    cform: None,
                }),
                word_next: None,
                accent_phrase_prev: None,
                accent_phrase_curr: Some(AccentPhraseCurrent {
                    mora_count: 5,
                    accent_position: 5,
                    is_interrogative: false,
                    accent_phrase_position_forward: 1,
                    accent_phrase_position_backward: 1,
                    mora_position_forward: 1,
                    mora_position_backward: 5,
                }),
                accent_phrase_next: None,
                breath_group_prev: None,
                breath_group_curr: Some(BreathGroupCurrent {
                    accent_phrase_count: 1,
                    mora_count: 5,
                    breath_group_position_forward: 1,
                    breath_group_position_backward: 1,
                    accent_phrase_position_forward: 1,
                    accent_phrase_position_backward: 1,
                    mora_position_forward: 1,
                    mora_position_backward: 5,
                }),
                breath_group_next: None,
                utterance: Utterance {
                    breath_group_count: 1,
                    accent_phrase_count: 1,
                    mora_count: 5,
                },
            },
        ),
        (
            "o^N-n+i=ch/A:-2+3+3/B:xx-xx_xx/C:09_xx+xx/D:xx+xx_xx/E:xx_xx!xx_xx-xx/F:5_5#0_xx@1_1|1_5/G:xx_xx%xx_xx_xx/H:xx_xx/I:1-5@1+1&1-1|1+5/J:xx_xx/K:1+1-5",
            Label {
                phoneme: Phoneme {
                    p2: Some("o".to_string()),
                    p1: Some("N".to_string()),
                    c: Some("n".to_string()),
                    n1: Some("i".to_string()),
                    n2: Some("ch".to_string()),
                },
                mora: Some(Mora {
                    relative_accent_position: -2,
                    position_forward: 3,
                    position_backward: 3,
                }),
                word_prev: None,
                word_curr: Some(Word {
                    pos: Some(9),
                    ctype: None,
                    cform: None,
                }),
                word_next: None,
                accent_phrase_prev: None,
                accent_phrase_curr: Some(AccentPhraseCurrent {
                    mora_count: 5,
                    accent_position: 5,
                    is_interrogative: false,
                    accent_phrase_position_forward: 1,
                    accent_phrase_position_backward: 1,
                    mora_position_forward: 1,
                    mora_position_backward: 5,
                }),
                accent_phrase_next: None,
                breath_group_prev: None,
                breath_group_curr: Some(BreathGroupCurrent {
                    accent_phrase_count: 1,
                    mora_count: 5,
                    breath_group_position_forward: 1,
                    breath_group_position_backward: 1,
                    accent_phrase_position_forward: 1,
                    accent_phrase_position_backward: 1,
                    mora_position_forward: 1,
                    mora_position_backward: 5,
                }),
                breath_group_next: None,
                utterance: Utterance {
                    breath_group_count: 1,
                    accent_phrase_count: 1,
                    mora_count: 5,
                },
            },
        ),
        (
            "N^n-i+ch=i/A:-2+3+3/B:xx-xx_xx/C:09_xx+xx/D:xx+xx_xx/E:xx_xx!xx_xx-xx/F:5_5#0_xx@1_1|1_5/G:xx_xx%xx_xx_xx/H:xx_xx/I:1-5@1+1&1-1|1+5/J:xx_xx/K:1+1-5",
            Label {
                phoneme: Phoneme {
                    p2: Some("N".to_string()),
                    p1: Some("n".to_string()),
                    c: Some("i".to_string()),
                    n1: Some("ch".to_string()),
                    n2: Some("i".to_string()),
                },
                mora: Some(Mora {
                    relative_accent_position: -2,
                    position_forward: 3,
                    position_backward: 3,
                }),
                word_prev: None,
                word_curr: Some(Word {
                    pos: Some(9),
                    ctype: None,
                    cform: None,
                }),
                word_next: None,
                accent_phrase_prev: None,
                accent_phrase_curr: Some(AccentPhraseCurrent {
                    mora_count: 5,
                    accent_position: 5,
                    is_interrogative: false,
                    accent_phrase_position_forward: 1,
                    accent_phrase_position_backward: 1,
                    mora_position_forward: 1,
                    mora_position_backward: 5,
                }),
                accent_phrase_next: None,
                breath_group_prev: None,
                breath_group_curr: Some(BreathGroupCurrent {
                    accent_phrase_count: 1,
                    mora_count: 5,
                    breath_group_position_forward: 1,
                    breath_group_position_backward: 1,
                    accent_phrase_position_forward: 1,
                    accent_phrase_position_backward: 1,
                    mora_position_forward: 1,
                    mora_position_backward: 5,
                }),
                breath_group_next: None,
                utterance: Utterance {
                    breath_group_count: 1,
                    accent_phrase_count: 1,
                    mora_count: 5,
                },
            },
        ),
        (
            "n^i-ch+i=w/A:-1+4+2/B:xx-xx_xx/C:09_xx+xx/D:xx+xx_xx/E:xx_xx!xx_xx-xx/F:5_5#0_xx@1_1|1_5/G:xx_xx%xx_xx_xx/H:xx_xx/I:1-5@1+1&1-1|1+5/J:xx_xx/K:1+1-5",
            Label {
                phoneme: Phoneme {
                    p2: Some("n".to_string()),
                    p1: Some("i".to_string()),
                    c: Some("ch".to_string()),
                    n1: Some("i".to_string()),
                    n2: Some("w".to_string()),
                },
                mora: Some(Mora {
                    relative_accent_position: -1,
                    position_forward: 4,
                    position_backward: 2,
                }),
                word_prev: None,
                word_curr: Some(Word {
                    pos: Some(9),
                    ctype: None,
                    cform: None,
                }),
                word_next: None,
                accent_phrase_prev: None,
                accent_phrase_curr: Some(AccentPhraseCurrent {
                    mora_count: 5,
                    accent_position: 5,
                    is_interrogative: false,
                    accent_phrase_position_forward: 1,
                    accent_phrase_position_backward: 1,
                    mora_position_forward: 1,
                    mora_position_backward: 5,
                }),
                accent_phrase_next: None,
                breath_group_prev: None,
                breath_group_curr: Some(BreathGroupCurrent {
                    accent_phrase_count: 1,
                    mora_count: 5,
                    breath_group_position_forward: 1,
                    breath_group_position_backward: 1,
                    accent_phrase_position_forward: 1,
                    accent_phrase_position_backward: 1,
                    mora_position_forward: 1,
                    mora_position_backward: 5,
                }),
                breath_group_next: None,
                utterance: Utterance {
                    breath_group_count: 1,
                    accent_phrase_count: 1,
                    mora_count: 5,
                },
            },
        ),
        (
            "i^ch-i+w=a/A:-1+4+2/B:xx-xx_xx/C:09_xx+xx/D:xx+xx_xx/E:xx_xx!xx_xx-xx/F:5_5#0_xx@1_1|1_5/G:xx_xx%xx_xx_xx/H:xx_xx/I:1-5@1+1&1-1|1+5/J:xx_xx/K:1+1-5",
            Label {
                phoneme: Phoneme {
                    p2: Some("i".to_string()),
                    p1: Some("ch".to_string()),
                    c: Some("i".to_string()),
                    n1: Some("w".to_string()),
                    n2: Some("a".to_string()),
                },
                mora: Some(Mora {
                    relative_accent_position: -1,
                    position_forward: 4,
                    position_backward: 2,
                }),
                word_prev: None,
                word_curr: Some(Word {
                    pos: Some(9),
                    ctype: None,
                    cform: None,
                }),
                word_next: None,
                accent_phrase_prev: None,
                accent_phrase_curr: Some(AccentPhraseCurrent {
                    mora_count: 5,
                    accent_position: 5,
                    is_interrogative: false,
                    accent_phrase_position_forward: 1,
                    accent_phrase_position_backward: 1,
                    mora_position_forward: 1,
                    mora_position_backward: 5,
                }),
                accent_phrase_next: None,
                breath_group_prev: None,
                breath_group_curr: Some(BreathGroupCurrent {
                    accent_phrase_count: 1,
                    mora_count: 5,
                    breath_group_position_forward: 1,
                    breath_group_position_backward: 1,
                    accent_phrase_position_forward: 1,
                    accent_phrase_position_backward: 1,
                    mora_position_forward: 1,
                    mora_position_backward: 5,
                }),
                breath_group_next: None,
                utterance: Utterance {
                    breath_group_count: 1,
                    accent_phrase_count: 1,
                    mora_count: 5,
                },
            },
        ),
        (
            "ch^i-w+a=sil/A:0+5+1/B:xx-xx_xx/C:09_xx+xx/D:xx+xx_xx/E:xx_xx!xx_xx-xx/F:5_5#0_xx@1_1|1_5/G:xx_xx%xx_xx_xx/H:xx_xx/I:1-5@1+1&1-1|1+5/J:xx_xx/K:1+1-5",
            Label {
                phoneme: Phoneme {
                    p2: Some("ch".to_string()),
                    p1: Some("i".to_string()),
                    c: Some("w".to_string()),
                    n1: Some("a".to_string()),
                    n2: Some("sil".to_string()),
                },
                mora: Some(Mora {
                    relative_accent_position: 0,
                    position_forward: 5,
                    position_backward: 1,
                }),
                word_prev: None,
                word_curr: Some(Word {
                    pos: Some(9),
                    ctype: None,
                    cform: None,
                }),
                word_next: None,
                accent_phrase_prev: None,
                accent_phrase_curr: Some(AccentPhraseCurrent {
                    mora_count: 5,
                    accent_position: 5,
                    is_interrogative: false,
                    accent_phrase_position_forward: 1,
                    accent_phrase_position_backward: 1,
                    mora_position_forward: 1,
                    mora_position_backward: 5,
                }),
                accent_phrase_next: None,
                breath_group_prev: None,
                breath_group_curr: Some(BreathGroupCurrent {
                    accent_phrase_count: 1,
                    mora_count: 5,
                    breath_group_position_forward: 1,
                    breath_group_position_backward: 1,
                    accent_phrase_position_forward: 1,
                    accent_phrase_position_backward: 1,
                    mora_position_forward: 1,
                    mora_position_backward: 5,
                }),
                breath_group_next: None,
                utterance: Utterance {
                    breath_group_count: 1,
                    accent_phrase_count: 1,
                    mora_count: 5,
                },
            },
        ),
        (
            "i^w-a+sil=xx/A:0+5+1/B:xx-xx_xx/C:09_xx+xx/D:xx+xx_xx/E:xx_xx!xx_xx-xx/F:5_5#0_xx@1_1|1_5/G:xx_xx%xx_xx_xx/H:xx_xx/I:1-5@1+1&1-1|1+5/J:xx_xx/K:1+1-5",
            Label {
                phoneme: Phoneme {
                    p2: Some("i".to_string()),
                    p1: Some("w".to_string()),
                    c: Some("a".to_string()),
                    n1: Some("sil".to_string()),
                    n2: None,
                },
                mora: Some(Mora {
                    relative_accent_position: 0,
                    position_forward: 5,
                    position_backward: 1,
                }),
                word_prev: None,
                word_curr: Some(Word {
                    pos: Some(9),
                    ctype: None,
                    cform: None,
                }),
                word_next: None,
                accent_phrase_prev: None,
                accent_phrase_curr: Some(AccentPhraseCurrent {
                    mora_count: 5,
                    accent_position: 5,
                    is_interrogative: false,
                    accent_phrase_position_forward: 1,
                    accent_phrase_position_backward: 1,
                    mora_position_forward: 1,
                    mora_position_backward: 5,
                }),
                accent_phrase_next: None,
                breath_group_prev: None,
                breath_group_curr: Some(BreathGroupCurrent {
                    accent_phrase_count: 1,
                    mora_count: 5,
                    breath_group_position_forward: 1,
                    breath_group_position_backward: 1,
                    accent_phrase_position_forward: 1,
                    accent_phrase_position_backward: 1,
                    mora_position_forward: 1,
                    mora_position_backward: 5,
                }),
                breath_group_next: None,
                utterance: Utterance {
                    breath_group_count: 1,
                    accent_phrase_count: 1,
                    mora_count: 5,
                },
            },
        ),
        (
            "w^a-sil+xx=xx/A:xx+xx+xx/B:xx-xx_xx/C:xx_xx+xx/D:xx+xx_xx/E:5_5!0_xx-xx/F:xx_xx#xx_xx@xx_xx|xx_xx/G:xx_xx%xx_xx_xx/H:1_5/I:xx-xx@xx+xx&xx-xx|xx+xx/J:xx_xx/K:1+1-5",
            Label {
                phoneme: Phoneme {
                    p2: Some("w".to_string()),
                    p1: Some("a".to_string()),
                    c: Some("sil".to_string()),
                    n1: None,
                    n2: None,
                },
                mora: None,
                word_prev: None,
                word_curr: None,
                word_next: None,
                accent_phrase_prev: Some(AccentPhrasePrevNext {
                    mora_count: 5,
                    accent_position: 5,
                    is_interrogative: false,
                    is_pause_insertion: None,
                }),
                accent_phrase_curr: None,
                accent_phrase_next: None,
                breath_group_prev: Some(BreathGroupPrevNext {
                    accent_phrase_count: 1,
                    mora_count: 5,
                }),
                breath_group_curr: None,
                breath_group_next: None,
                utterance: Utterance {
                    breath_group_count: 1,
                    accent_phrase_count: 1,
                    mora_count: 5,
                },
            },
        ),
        // 「なにを言っているのですか，それはスマホですよ．」
        // (partial; 6th phoneme including the first sil)
        (
            "n^i-o+i=cl/A:2+3+1/B:04-xx_xx/C:13_xx+xx/D:20+1_1/E:xx_xx!xx_xx-xx/F:3_1#0_xx@1_4|1_12/G:3_3%0_xx_1/H:xx_xx/I:4-12@1+2&1-6|1+21/J:2_9/K:2+6-21",
            Label {
                phoneme: Phoneme {
                    p2: Some("n".to_string()),
                    p1: Some("i".to_string()),
                    c: Some("o".to_string()),
                    n1: Some("i".to_string()),
                    n2: Some("cl".to_string()),
                },
                mora: Some(Mora {
                    relative_accent_position: 2,
                    position_forward: 3,
                    position_backward: 1,
                }),
                word_prev: Some(Word {
                    pos: Some(4),
                    ctype: None,
                    cform: None,
                }),
                word_curr: Some(Word {
                    pos: Some(13),
                    ctype: None,
                    cform: None,
                }),
                word_next: Some(Word {
                    pos: Some(20),
                    ctype: Some(1),
                    cform: Some(1),
                }),
                accent_phrase_prev: None,
                accent_phrase_curr: Some(AccentPhraseCurrent {
                    mora_count: 3,
                    accent_position: 1,
                    is_interrogative: false,
                    accent_phrase_position_forward: 1,
                    accent_phrase_position_backward: 4,
                    mora_position_forward: 1,
                    mora_position_backward: 12,
                }),
                accent_phrase_next: Some(AccentPhrasePrevNext {
                    mora_count: 3,
                    accent_position: 3,
                    is_interrogative: false,
                    is_pause_insertion: Some(false),
                }),
                breath_group_prev: None,
                breath_group_curr: Some(BreathGroupCurrent {
                    accent_phrase_count: 4,
                    mora_count: 12,
                    breath_group_position_forward: 1,
                    breath_group_position_backward: 2,
                    accent_phrase_position_forward: 1,
                    accent_phrase_position_backward: 6,
                    mora_position_forward: 1,
                    mora_position_backward: 21,
                }),
                breath_group_next: Some(BreathGroupPrevNext {
                    accent_phrase_count: 2,
                    mora_count: 9,
                }),
                utterance: Utterance {
                    breath_group_count: 2,
                    accent_phrase_count: 6,
                    mora_count: 21,
                },
            },
        ),
    ]
}
