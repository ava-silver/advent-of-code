import gleam/int
import gleam/list
import gleam/string

// const test_input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124"

const input = "1061119-1154492,3-23,5180469-5306947,21571-38630,1054-2693,141-277,2818561476-2818661701,21177468-21246892,40-114,782642-950030,376322779-376410708,9936250-10074071,761705028-761825622,77648376-77727819,2954-10213,49589608-49781516,9797966713-9797988709,4353854-4515174,3794829-3861584,7709002-7854055,7877419320-7877566799,953065-1022091,104188-122245,25-39,125490-144195,931903328-931946237,341512-578341,262197-334859,39518-96428,653264-676258,304-842,167882-252124,11748-19561"

/// Parse the input to a list of moves
fn parse(input: String) -> List(#(Int, Int)) {
  string.split(input, ",")
  |> list.filter_map(fn(e) {
    case string.split(e, "-") |> list.map(int.parse) {
      [Ok(a), Ok(b)] -> Ok(#(a, b))
      _ -> Error(Nil)
    }
  })
}

pub fn main() {
  let data = parse(input)
  // part 1
  let invalid_ids =
    list.map(data, fn(r) {
      list.range(r.0, r.1)
      |> list.filter(fn(x) {
        let str = int.to_string(x)
        let len = string.length(str)
        string.slice(str, 0, len / 2) == string.slice(str, len / 2, len)
      })
      |> int.sum()
    })
    |> int.sum()
  echo invalid_ids

  // part 2
  let invalid_ids =
    list.map(data, fn(r) {
      list.range(r.0, r.1)
      |> list.filter(fn(x) {
        let str = int.to_string(x)
        let letters = string.to_graphemes(str)
        list.range(1, string.length(str) / 2)
        |> list.any(fn(l) {
          let chunks = list.sized_chunk(letters, l)
          let assert Ok(first_chunk) = list.first(chunks)
          list.length(chunks) > 1
          && list.all(chunks, fn(chunk) { chunk == first_chunk })
        })
      })
      |> int.sum()
    })
    |> int.sum()
  echo invalid_ids
}
