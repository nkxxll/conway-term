use crate::game::State;

/// convert a game state to an run length encoded blob
pub(crate) fn state_to_rle(state: &State) -> Vec<u8> {
    let mut flatten_state = state.state.clone().into_iter().flatten();
    let mut res: Vec<u8> = vec![];
    let mut count: u8 = 1;
    let mut current = match flatten_state.next() {
        Some(x) => x,
        None => {
            return res;
        }
    };
    for next in flatten_state {
        if next == current {
            assert!(
                count as u16 + 1 < 2_u16.pow(8),
                "big upsii with the rle the count is bigger than a byte"
            );
            count += 1;
            continue;
        }
        res.push(count);
        res.push(current as u8);
        current = next;
        count = 1;
    }
    // if there are some chars left but the before last does not differ the last then
    res.push(count);
    res.push(current as u8);
    res
}

pub(crate) fn state_from_rle(data: Vec<u8>, width: isize, height: isize) -> crate::game::State {
    let mut res: Vec<char> = vec![];
    for entry in data.chunks(2) {
        let count = *entry.get(0).unwrap();
        let char = *entry.get(1).unwrap();
        let add_vec = vec![char as char; count as usize];
        res = [res, add_vec].concat();
    }

    // split into columns
    let state: Vec<Vec<char>> = res.chunks(width as usize).map(|x| x.to_vec()).collect();

    State::new(state, width, height)
}

#[cfg(test)]
mod tests_rle {
    use super::*;
    use std::iter::zip;

    #[test]
    fn test_rle() {
        let mut state = vec![
            vec![
                ' ', ' ', '#', '#', '#', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
                ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
                ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
                ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
                ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
                ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
                ' ', ' ', ' ', ' ',
            ],
            vec![
                '#', '#', ' ', ' ', ' ', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#',
                '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#',
                '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#',
                '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#',
                '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#',
                '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#',
                '#', '#', '#', ' ',
            ],
        ];
        fn zipvec<T>(vec1: Vec<T>, vec2: Vec<T>) -> Vec<T> {
            let mut res = vec![];
            for (a, b) in zip(vec1, vec2) {
                res.push(a);
                res.push(b);
            }
            res
        }

        let add: Vec<char> = zipvec(vec!['#'; 50], vec![' '; 50]);
        assert!(add.len() == 100);

        let add_expected: Vec<u8> = zipvec(
            vec![1; 100],
            add.clone().into_iter().map(|x| x as u8).collect(),
        );

        state.push(add);

        let encoded = state_to_rle(&crate::game::State::new(state, 100, 3));
        let expected = vec![
            2, b' ', 3, b'#', 95, b' ', 2, b'#', 3, b' ', 94, b'#', 1, b' ',
        ];

        assert_eq!(encoded, [expected, add_expected].concat());
    }

    #[test]
    fn test_state_from_rle() {
        let mut state = crate::game::State::new(
            vec![
                vec![
                    ' ', ' ', '#', '#', '#', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
                    ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
                    ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
                    ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
                    ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
                    ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
                    ' ', ' ', ' ', ' ',
                ],
                vec![
                    '#', '#', ' ', ' ', ' ', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#',
                    '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#',
                    '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#',
                    '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#',
                    '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#',
                    '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#',
                    '#', '#', '#', ' ',
                ],
            ],
            100,
            3,
        );
        fn zipvec<T>(vec1: Vec<T>, vec2: Vec<T>) -> Vec<T> {
            let mut res = vec![];
            for (a, b) in zip(vec1, vec2) {
                res.push(a);
                res.push(b);
            }
            res
        }

        let add: Vec<char> = zipvec(vec!['#'; 50], vec![' '; 50]);
        assert!(add.len() == 100);

        let add_expected: Vec<u8> = zipvec(
            vec![1; 100],
            add.clone().into_iter().map(|x| x as u8).collect(),
        );

        state.state.push(add);

        let mut input_rle = vec![
            2, b' ', 3, b'#', 95, b' ', 2, b'#', 3, b' ', 94, b'#', 1, b' ',
        ];
        input_rle = [input_rle, add_expected].concat();
        let state_back = state_from_rle(input_rle, 100, 3);

        assert_eq!(state_back.state, state.state);
    }
}
