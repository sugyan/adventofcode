use 5.38.0;
use feature 'class';
no warnings 'experimental::class';

use Base;

class File {
    field $id       : param;
    field $position : param : reader;
    field $length   : param : reader;

    method calculate_checksum( $pos, $len ) {
        return $id * ( $pos + $pos + $len - 1 ) * $len / 2;
    }
}

class Solution : isa(Base) {
    use List::Util qw(min);

    method parse($lines) {
        return [ split //, $lines->[0] ];
    }

    method part1() {
        my $input = $self->input();
        return resulting_checksum( $input, 0 );
    }

    method part2() {
        my $input = $self->input();
        return resulting_checksum( $input, 1 );
    }

    sub extract_files_and_frees($input) {
        my @files = ();
        my @frees = map { [] } 0 .. 9;
        my $pos   = 0;
        while ( my ( $i, $digit ) = each $input->@* ) {
            if ( $i % 2 == 0 ) {
                push @files,
                  File->new(
                    id       => $i / 2,
                    position => $pos,
                    length   => $digit
                  );
            }
            elsif ( $digit > 0 ) {
                push $frees[$digit]->@*, $pos;
            }
            $pos += $digit;
        }
        return ( \@files, \@frees );
    }

    sub resulting_checksum( $input, $move_whole ) {
        my ( $files, $frees ) = extract_files_and_frees($input);
        my $sum = 0;
        for my $file ( reverse $files->@* ) {
            my $remain = $file->length;
            my $need   = $move_whole ? $file->length : 1;
            while ( $remain > 0 ) {
                my ( $best_s, $best_p ) = ( undef, $file->position );
                for my $s ( $need .. 9 ) {
                    next unless $frees->[$s]->@* && $frees->[$s][0] < $best_p;
                    ( $best_s, $best_p ) = ( $s, $frees->[$s][0] );
                }
                if ( defined $best_s ) {
                    my $take = min( $best_s, $remain );
                    $sum    += $file->calculate_checksum( $best_p, $take );
                    $remain -= $take;
                    my $p = shift $frees->[$best_s]->@*;
                    my $s = $best_s - $take;
                    if ( $s > 0 ) {
                        push $frees->[$s]->@*, $take + $p;
                        $frees->[$s]->@* = sort { $a <=> $b } $frees->[$s]->@*;
                    }
                }
                else {
                    $sum += $file->calculate_checksum( $best_p, $remain );
                    last;
                }
            }
        }
        return $sum;
    }
}
