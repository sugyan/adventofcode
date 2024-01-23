open Base
open Solution

module Solution : Solve = struct
  module Subset = struct
    type t = { r : int; g : int; b : int }

    let create r g b = { r; g; b }
    let zero = create 0 0 0
    let max x y = create (max x.r y.r) (max x.g y.g) (max x.b y.b)
    let ( >= ) x y = x.r >= y.r && x.g >= y.g && x.b >= y.b

    let of_string s =
      let f s =
        let num, color = String.(strip s |> rsplit2_exn ~on:' ') in
        match color with
        | "red" -> create (Int.of_string num) 0 0
        | "green" -> create 0 (Int.of_string num) 0
        | "blue" -> create 0 0 (Int.of_string num)
        | _ -> failwith "invalid color"
      in
      String.split s ~on:',' |> List.map ~f |> List.fold ~init:zero ~f:max

    let power t = t.r * t.g * t.b
  end

  type t = (int * Subset.t list) list

  let parse input =
    let parse_line line =
      let game, sets = String.lsplit2_exn line ~on:':' in
      ( String.chop_prefix_exn game ~prefix:"Game " |> Int.of_string,
        String.split sets ~on:';' |> List.map ~f:Subset.of_string )
    in
    In_channel.input_lines input |> List.map ~f:parse_line

  let part1 games =
    let f (_, subsets) =
      List.for_all subsets ~f:Subset.(( >= ) (create 12 13 14))
    in
    games |> List.filter ~f |> List.sum (module Int) ~f:fst |> Answer.of_int

  let part2 games =
    let f (_, subsets) =
      List.fold subsets ~init:Subset.zero ~f:Subset.max |> Subset.power
    in
    games |> List.sum (module Int) ~f |> Answer.of_int
end
