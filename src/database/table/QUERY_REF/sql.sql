OPERATION SELECTION
"""
select 
	op.label as operation_label,
    op.status as operation_status,
    concat(p.first_name, ' ', p.last_name) as full_name,
    r.name as room,
    r.alias_code as room_code
from 
	operation op
left join 
	patient p on op.patient_id = p.id
left join
	room r on op.room_id = r.id;
"""

