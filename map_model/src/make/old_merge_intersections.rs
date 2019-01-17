use crate::raw_data;
use abstutil::Timer;

pub fn old_merge_intersections(data: &mut raw_data::Map, _timer: &mut Timer) {
    /*if true {
        return;
    }*/

    // 15th and McGraw
    merge(data, raw_data::StableRoadID(59));

    // 14th and Boston
    merge(data, raw_data::StableRoadID(389));
    merge(data, raw_data::StableRoadID(22));

    // TODO When we want to find the roads to do this automatically, we can't use original road
    // length, since it effectively changes while we delete intersections...
}

fn merge(data: &mut raw_data::Map, merge_road: raw_data::StableRoadID) {
    // Arbitrarily kill off the first intersection and keep the second one.
    let (delete_i, keep_i) = {
        let r = data.roads.remove(&merge_road).unwrap();
        (r.i1, r.i2)
    };
    data.intersections.remove(&delete_i);

    for r in data.roads.values_mut() {
        if r.i1 == delete_i {
            r.i1 = keep_i;
        }
        if r.i2 == delete_i {
            r.i2 = keep_i;
        }
    }

    // TODO retain for btreemap, please!
    let mut remove_roads: Vec<raw_data::StableRoadID> = Vec::new();
    for (id, r) in &data.roads {
        if r.i1 == r.i2 {
            remove_roads.push(*id);
        }
    }
    for id in remove_roads {
        data.roads.remove(&id);
    }
    // TODO Not exactly sure WHEN this happens, but we can wind up creating some loop roads...
    // filter them out.
    // TODO Ah, we can also wind up with multiple roads between the same intersections here. Should
    // probably auto-remove those too.
}
