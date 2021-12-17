<?php declare(strict_types=1);
use PHPUnit\Framework\TestCase;

final class SmokeBasinTest extends TestCase
{
    public function test_make_window(): void
    {
        $input = array(array(1, 0), array(2, 1));
        $window = new Window($input, 0, 0);
        $this->assertEquals(
            $window->get(),
            array(
                array(9, 9, 9),
                array(9, 1, 0),
                array(9, 2, 1)
            )
        );
    }

    public function test_is_low_point_false(): void
    {
        $input = array(array(3, 0), array(2, 1));
        $window = new Window($input, 0, 0);
        $low_point = $window->is_low_point();
        $this->assertEquals(
            $low_point,
            false
        );
    }

    public function test_is_low_point_true(): void
    {
        $input = array(
            array(1, 0),
            array(2, 1)
        );
        $window = new Window($input, 0, 1);
        $low_point = $window->is_low_point();
        $this->assertEquals(
            $low_point,
            true
        );
    }

    public function test_get_low_points(): void
    {
        $input = array(
            array(1, 0),
            array(2, 1)
        );
        $actual = SmokeBasin::get_low_points($input);
        $this->assertEquals(
            1,
            count($actual)
        );
        $this->assertEquals($actual[0], array(0,1));
    }

    public function test_get_low_points_when_adjacent_equal(): void
    {
        $input = array(
            array(1, 1),
            array(1, 1)
        );
        $actual = SmokeBasin::get_low_points($input);
        $this->assertEquals(
            0,
            count($actual)
        );
    }

    public function test_parse_height_map(): void
    {
        $actual = HeightMap::from_file("./input-test.txt");
        $this->assertEquals(count($actual), 5);
        $this->assertEquals(count($actual[0]), 10);
        $this->assertEquals($actual[0][9], 0);
    }

    public function test_parse_height_map_part1(): void
    {
        $actual = HeightMap::from_file("./input.txt");
        $this->assertEquals(count($actual), 100);
        $this->assertEquals(count($actual[0]), 100);
    }

    public function test_get_risk_level(): void
    {
        $height_map = array(
            array(1,2,3),
            array(1,2,3),
            array(1,2,3),
        );
        $actual = RiskLevel::get(array(array(1,0)), $height_map);
        $this->assertEquals($actual, 2);
    }

    public function test_get_risk_level_multiple(): void
    {
        $height_map = array(
            array(1,2,3),
            array(1,2,3),
            array(1,2,3),
        );
        $actual = RiskLevel::get(array(array(1,0), array(0, 2)), $height_map);
        $this->assertEquals($actual, 6);
    }

    public function test_low_points_from_file(): void
    {
        $actual = SmokeBasin::get_low_points_from_file("./input-test.txt");
        $this->assertEquals(count($actual), 4);
        $this->assertEquals($actual[0], array(0, 1));
        $this->assertEquals($actual[1], array(0, 9));
        $this->assertEquals($actual[2], array(2, 2));
        $this->assertEquals($actual[3], array(4, 6));
    }

    public function test_get_total_risk_level(): void
    {
        $actual = SmokeBasin::get_total_risk_level("./input-test.txt");
        $this->assertEquals($actual, 15);
    }

    public function test_part_1(): void
    {
        $actual = SmokeBasin::get_total_risk_level("./input.txt");
        $this->assertEquals(496, $actual);
    }

    public function test_finding_a_basin_simple() {
        $height_map = array(
            array(9,9,9),
            array(9,2,9),
            array(9,9,9),
        );
        $basins = SmokeBasin::get_basins($height_map);

        $this->assertEquals(count($basins), 1);
    }

    public function test_adjacent_windows() {
        $height_map = array(
            array(9,9,9,3),
            array(9,2,9,9),
            array(9,9,9,9),
        );
        $window = new Window($height_map, 1, 1);
        $actual = $window->is_next_to(new Window($height_map, 3, 0));
        $this->assertEquals(false, $actual);
    }

    public function test_vertically_adjacent_windows() {
        $height_map = array(
            array(9,1,9,3),
            array(1,2,9,9),
            array(9,9,9,9),
        );
        $window = new Window($height_map, 1, 1);
        $actual = $window->is_next_to(new Window($height_map, 0, 1));
        $this->assertEquals(true, $actual);
        $window_a = new Window($height_map, 0, 1);
        $this->assertEquals($window_a->is_next_to(new Window($height_map,1, 1)), true);
    }

    public function test_finding_a_basin_multiple() {
        $height_map = array(
            array(9,9,9,3),
            array(9,2,9,9),
            array(9,9,9,9),
        );
        $basins = SmokeBasin::get_basins($height_map);

        $this->assertEquals(count($basins), 2);
        $this->assertEquals(count($basins[0]->get_windows()), 1);
        $this->assertEquals(count($basins[1]->get_windows()), 1);
    }

    public function test_finding_a_basin_spanning_multiple_spots() {
        $height_map = array(
            array(9,9,9,9),
            array(9,3,7,9),
            array(9,9,9,9),
        );
        $basins = SmokeBasin::get_basins($height_map);

        $this->assertEquals(1, count($basins));
        $this->assertEquals(2, count($basins[0]->get_windows()));
        $windows = $basins[0]->get_windows();
        $this->assertEquals(
            array_map(function ($window) {
                return $window->point();
            }, $windows),
            array(
                array(1, 1),
                array(1, 2)
            )
        );
    }

    public function test_finding_a_basin_forming_a_corner() {
        $height_map = array(
            array(9,9,2,9),
            array(9,3,7,9),
            array(9,9,9,9),
        );
        $basins = SmokeBasin::get_basins($height_map);

        $this->assertEquals(1, count($basins));
        $this->assertEquals(3, count($basins[0]->get_windows()));
        $this->assertEquals(array_map(function ($window) {
            return $window->point();
        }, $basins[0]->get_windows()), array(
            array(1, 2),
            array(1, 1),
            array(0, 2),
        ));
    }

    public function test_in_array() {
        $test = array(array(1,2), array(2,3));
        $this->assertEquals(in_array(array(1,2), $test), true);
        $this->assertEquals(in_array(array(2,3), $test), true);

    }

    public function test_part_2() {
        $height_map = HeightMap::from_file("./input-test.txt");
        $basins = SmokeBasin::get_basins($height_map);
        $this->assertEquals(4, count($basins));
    }

    public function test_merge_algorigthm() {
        $basins = [[1,2],[2],[3], [3,4]];
        $merged = [$basins[0]];
        for ($i = 0; $i < count($basins); $i++) {
            $basin = $basins[$i];
            $exists_in_index = null;
            for ($j = 0; $j < count($merged); $j++) {
                $merged_basin = $merged[$j];
                $exists = array_some($basin, function($point) use($merged_basin) {
                    return in_array($point, $merged_basin);
                });
                if ($exists) {
                    $exists_in_index = $j;
                }
            }
            if (!is_null($exists_in_index)) {
                $merged[$exists_in_index] = unique($merged[$exists_in_index], $basin);
            } else {
                array_push($merged, $basin);
            }
        }
        $expected = [[1,2], [3,4]];
        $this->assertEquals(count($merged), 2);
        $this->assertEquals($expected, $merged);
    }
}

function unique(array $a, array $b) {
    $b_not_in_a = array_filter($b, function($element) use($a) {
        return !in_array($element, $a);
    });
    return [...$a, ...$b_not_in_a];
}

function array_any(array $array, Callable $predicate) {
    return count(array_filter($array, $predicate)) > 0;
}

function array_every(array $array, Callable $predicate): bool {
    return array_reduce($array, function($every, $el) use ($predicate) {
        return $every && $predicate($el);
    }, true);
}

function array_some(array $array, Callable $predicate): bool {
    return array_reduce($array, function($every, $el) use ($predicate) {
        return $every || $predicate($el);
    }, false);
}