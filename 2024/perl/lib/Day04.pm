use 5.38.0;
use feature 'class';
no warnings 'experimental::class';

use Base;

class Solution : isa(Base) {
    use List::Util qw(sum);

    method parse($lines) {
        return {
            lines => join( "\n", $lines->@* ),
            width => length( $lines->[0] ),
        };
    }

    method part1() {
        my $input = $self->input();
        my $width = $input->{width};
        return sum map { scalar( () = $input->{lines} =~ /$_/g ) } (
            ( qr/XMAS/, qr/SAMX/ ),
            ( map { qr/(?=X.{$_}M.{$_}A.{$_}S)/s } $width - 1 .. $width + 1 ),
            ( map { qr/(?=S.{$_}A.{$_}M.{$_}X)/s } $width - 1 .. $width + 1 ),
        );
    }

    method part2() {
        my $input = $self->input();
        my $w     = $input->{width} - 1;
        return sum map { scalar( () = $input->{lines} =~ /$_/g ) } (
            qr/(?=M.M.{$w}A.{$w}S.S)/s, qr/(?=S.S.{$w}A.{$w}M.M)/s,
            qr/(?=M.S.{$w}A.{$w}M.S)/s, qr/(?=S.M.{$w}A.{$w}S.M)/s,
        );
    }
}
