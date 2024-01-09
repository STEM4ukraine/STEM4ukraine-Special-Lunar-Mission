ha:cschem-sheet-v1 {
	ha:obj_indirect.1 {
		li:objects {
			ha:group.1 {
				uuid=4/pORh4XHs+cI/veJoAAAAAn;
				li:objects {
					ha:group.1 {
						uuid=4/pORh4XHs+cI/veJoAAAAAo; loclib_name=led5;
						li:objects {
						}
						ha:attrib {
							device=led5
							footprint=LED5
							li:portmap {
								{C->pcb/pinnum=1}
								{A->pcb/pinnum=2}
							}
						}
					}
					ha:group.2 {
						uuid=4/pORh4XHs+cI/veJoAAAADn; loclib_name=3mmLED_pth;
						li:objects {
						}
						ha:attrib {
							footprint=LED3.fp
							li:portmap {
								{C->pcb/pinnum=1}
								{A->pcb/pinnum=2}
							}
						}
					}
					ha:group.3 {
						uuid=8SwlWRR8ZyrHMrrHm7MAAABb; loclib_name=3mmLED_pth;
						li:objects {
						}
						ha:attrib {
							footprint=LED3.fp
							li:portmap {
								{C->pcb/pinnum=1}
								{A->pcb/pinnum=2}
							}
						}
					}
				}
				ha:attrib {
					ha:purpose = { value=devmap; prio=0; }
				}
			}
		}
	}
	ha:obj_direct.2 {
		uuid=4/pORh4XHs+cI/veJoAAAAAC;
		li:objects {
			ha:pen.sheet-decor { shape=round; size=125; color=#777777; font_height=3000; font_family=sans; }
			ha:pen.sheet-decor-fill { shape=round; size=125; color=#bbbbbb; font_height=3000; font_family=sans; }
			ha:pen.titlebox-frame { shape=round; size=250; color=#777777; font_height=0; }
			ha:pen.titlebox-fill { shape=round; size=250; color=#bbffbb; font_height=0; }
			ha:pen.titlebox-big { shape=round; size=250; color=#777777; font_height=3000; font_family=sans; }
			ha:pen.titlebox-small { shape=round; size=250; color=#777777; font_height=1500; font_family=sans; }
			ha:pen.wire { shape=round; size=250; color=#2222bb; font_height=3000; font_family=sans; }
			ha:pen.bus { shape=round; size=1500; color=#2222bb; font_height=3000; font_family=sans; }
			ha:pen.hub { shape=round; size=3000; color=#6666ff; font_height=3000; font_family=sans; }
			ha:pen.sym-decor { shape=round; size=125; color=#119911; font_height=3000; font_family=sans; }
			ha:pen.sym-decor-fill { shape=round; size=125; color=#99ff99; font_height=3000; font_family=sans; }
			ha:pen.sym-primary { shape=round; size=125; color=#119911; font_height=3000; font_family=sans; font_style=bold; }
			ha:pen.sym-secondary { shape=round; size=125; color=#33bb33; font_height=3000; font_family=sans; }
			ha:pen.term-decor { shape=round; size=250; color=#222222; font_height=3000; font_family=sans; }
			ha:pen.term-primary { shape=round; size=250; color=#222222; font_height=3000; font_family=sans; font_style=bold; }
			ha:pen.term-secondary { shape=round; size=250; color=#555555; font_height=3000; font_family=sans; }
			ha:pen.busterm-decor { shape=round; size=1500; color=#222222; font_height=3000; font_family=sans; }
			ha:pen.busterm-primary { shape=round; size=1500; color=#222222; font_height=3000; font_family=sans; font_style=bold; }
			ha:pen.busterm-secondary { shape=round; size=1500; color=#555555; font_height=3000; font_family=sans; }
			ha:pen.junction { shape=round; size=1000; color=#2222bb; font_height=3000; font_family=sans; }
			ha:group.2 {
				uuid=4/pORh4XHs+cI/veJoAAAAAV; src_uuid=/iiShtebwvwwWnNJ68YAAAAJ;
				x=52000; y=120000;
				li:objects {
					ha:text.1 { x1=0; y1=-8000; dyntext=1; stroke=sym-primary; text=%../A.name%; floater=1; }
					ha:polygon.2 {
						li:outline {
							ha:line { x1=0; y1=0; x2=0; y2=28000; }
							ha:line { x1=0; y1=28000; x2=16000; y2=28000; }
							ha:line { x1=16000; y1=28000; x2=16000; y2=0; }
							ha:line { x1=16000; y1=0; x2=0; y2=0; }
						}
						stroke=sym-decor;
					}
					ha:group.3 {
						uuid=4/pORh4XHs+cI/veJoAAAAAW; src_uuid=/iiShtebwvwwWnNJ68YAAAAB;
						x=16000; y=24000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
							ha:text.3 { x1=-500; y1=-2000; mirx=1; dyntext=1; stroke=term-secondary; text=%../A.name%; }
						}
						ha:attrib {
							name=PB0
							pinnum=5
							role=terminal
						}
					}
					ha:group.4 {
						uuid=4/pORh4XHs+cI/veJoAAAAAX; src_uuid=/iiShtebwvwwWnNJ68YAAAAC;
						x=16000; y=20000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
							ha:text.3 { x1=-500; y1=-2000; mirx=1; dyntext=1; stroke=term-secondary; text=%../A.name%; }
						}
						ha:attrib {
							name=PB1
							pinnum=6
							role=terminal
						}
					}
					ha:group.5 {
						uuid=4/pORh4XHs+cI/veJoAAAAAY; src_uuid=/iiShtebwvwwWnNJ68YAAAAD;
						x=16000; y=16000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
							ha:text.3 { x1=-500; y1=-2000; mirx=1; dyntext=1; stroke=term-secondary; text=%../A.name%; }
						}
						ha:attrib {
							name=PB2
							pinnum=7
							role=terminal
						}
					}
					ha:group.6 {
						uuid=4/pORh4XHs+cI/veJoAAAAAZ; src_uuid=/iiShtebwvwwWnNJ68YAAAAE;
						x=16000; y=12000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
							ha:text.3 { x1=-500; y1=-2000; mirx=1; dyntext=1; stroke=term-secondary; text=%../A.name%; }
						}
						ha:attrib {
							name=PB3
							pinnum=2
							role=terminal
						}
					}
					ha:group.7 {
						uuid=4/pORh4XHs+cI/veJoAAAAAa; src_uuid=/iiShtebwvwwWnNJ68YAAAAF;
						x=16000; y=8000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
							ha:text.3 { x1=-500; y1=-2000; mirx=1; dyntext=1; stroke=term-secondary; text=%../A.name%; }
						}
						ha:attrib {
							name=PB4
							pinnum=3
							role=terminal
						}
					}
					ha:group.8 {
						uuid=4/pORh4XHs+cI/veJoAAAAAb; src_uuid=/iiShtebwvwwWnNJ68YAAAAG;
						x=16000; y=4000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
							ha:text.3 { x1=-500; y1=-2000; mirx=1; dyntext=1; stroke=term-secondary; text=%../A.name%; }
						}
						ha:attrib {
							name=PB5
							pinnum=1
							role=terminal
						}
					}
					ha:group.9 {
						uuid=4/pORh4XHs+cI/veJoAAAAAc; src_uuid=/iiShtebwvwwWnNJ68YAAAAH;
						x=8000; y=28000; rot=90.000000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
							ha:text.3 { x1=-500; y1=-2000; mirx=1; dyntext=1; stroke=term-secondary; text=%../A.name%; }
						}
						ha:attrib {
							name=VCC
							pinnum=8
							role=terminal
						}
					}
					ha:group.10 {
						uuid=4/pORh4XHs+cI/veJoAAAAAd; src_uuid=/iiShtebwvwwWnNJ68YAAAAI;
						x=8000; y=0; rot=-90.000000; mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
							ha:text.3 { x1=-500; y1=-2000; mirx=1; dyntext=1; stroke=term-secondary; text=%../A.name%; }
						}
						ha:attrib {
							name=GND
							pinnum=4
							role=terminal
						}
					}
				}
				ha:attrib {
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-symbol-generator=boxsym-rnd
					footprint=dip(8)
					name=U1
					role=symbol
				}
			}
			ha:group.5 {
				uuid=4/pORh4XHs+cI/veJoAAAAA4; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAQ;
				x=120000; y=84000; rot=270.000000;
				li:objects {
					ha:group.1 {
						uuid=4/pORh4XHs+cI/veJoAAAAA5; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAR;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=C
							role=terminal
							ha:spice/pinnum = { value=2; prio=31050; }
						}
					}
					ha:group.2 {
						uuid=4/pORh4XHs+cI/veJoAAAAA6; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAS;
						x=-16000; y=0; mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=A
							role=terminal
							ha:spice/pinnum = { value=1; prio=31050; }
						}
					}
					ha:line.3 { x1=-4000; y1=0; x2=-6000; y2=0; stroke=sym-decor; }
					ha:line.4 { x1=-12000; y1=0; x2=-10000; y2=0; stroke=sym-decor; }
					ha:line.5 { x1=-10000; y1=4000; x2=-6000; y2=0; stroke=sym-decor; }
					ha:line.6 { x1=-6000; y1=0; x2=-10000; y2=-4000; stroke=sym-decor; }
					ha:line.7 { x1=-10000; y1=4000; x2=-10000; y2=-4000; stroke=sym-decor; }
					ha:line.8 { x1=-6000; y1=4000; x2=-6000; y2=-4000; stroke=sym-decor; }
					ha:text.9 { x1=-4000; y1=13000; rot=90.000000; dyntext=1; stroke=sym-secondary; text=%../a.devmap%; floater=1; }
					ha:text.10 { x1=-8000; y1=13000; rot=90.000000; dyntext=1; stroke=sym-primary; text=%../A.name%; floater=1; }
					ha:line.11 { x1=-8000; y1=8000; x2=-6000; y2=11000; stroke=sym-decor; }
					ha:line.12 { x1=-6000; y1=11000; x2=-7000; y2=10000; stroke=sym-decor; }
					ha:line.13 { x1=-6000; y1=11000; x2=-6517; y2=9545; stroke=sym-decor; }
					ha:line.14 { x1=-10000; y1=7000; x2=-8000; y2=10000; stroke=sym-decor; }
					ha:line.15 { x1=-8000; y1=10000; x2=-8000; y2=8000; stroke=sym-decor; }
					ha:line.16 { x1=-8303; y1=6354; x2=-6303; y2=9354; stroke=sym-decor; }
					ha:line.17 { x1=-6303; y1=9354; x2=-7303; y2=8354; stroke=sym-decor; }
					ha:line.18 { x1=-6303; y1=9354; x2=-6820; y2=7899; stroke=sym-decor; }
					ha:line.19 { x1=-10303; y1=5354; x2=-8303; y2=8354; stroke=sym-decor; }
					ha:line.20 { x1=-8303; y1=8354; x2=-8303; y2=6354; stroke=sym-decor; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					devmap=3mmLED_pth
					name=LED1
					role=symbol
					ha:spice/prefix = { value=D; prio=31050; }
				}
			}
			ha:group.6 {
				uuid=4/pORh4XHs+cI/veJoAAAAA7; src_uuid=iNOQfJpO6hT/HFDFGjoAAABC;
				x=120000; y=104000; rot=270.000000; miry=1;
				li:objects {
					ha:group.1 {
						uuid=4/pORh4XHs+cI/veJoAAAAA8; src_uuid=iNOQfJpO6hT/HFDFGjoAAABD;
						x=20000; y=0;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=2
							role=terminal
						}
					}
					ha:group.2 {
						uuid=4/pORh4XHs+cI/veJoAAAAA9; src_uuid=iNOQfJpO6hT/HFDFGjoAAABE;
						mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=1
							role=terminal
						}
					}
					ha:text.3 { x1=12000; y1=2000; rot=90.000000; dyntext=1; stroke=sym-primary; text=%../a.value%; floater=1; }
					ha:text.4 { x1=8000; y1=2000; rot=90.000000; dyntext=1; stroke=sym-primary; text=%../A.name%; floater=1; }
					ha:polygon.5 {
						li:outline {
							ha:line { x1=4000; y1=2000; x2=4000; y2=-2000; }
							ha:line { x1=4000; y1=-2000; x2=16000; y2=-2000; }
							ha:line { x1=16000; y1=-2000; x2=16000; y2=2000; }
							ha:line { x1=16000; y1=2000; x2=4000; y2=2000; }
						}
						stroke=sym-decor;
					}
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					ha:device = { value=resistor; prio=31050; }
					footprint=acy(300)
					name=R1
					role=symbol
					ha:spice/prefix = { value=R; prio=31050; }
					value=220R
				}
			}
			ha:group.7 {
				uuid=4/pORh4XHs+cI/veJoAAAAA+; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAQ;
				x=144000; y=84000; rot=270.000000;
				li:objects {
					ha:group.1 {
						uuid=4/pORh4XHs+cI/veJoAAAAA/; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAR;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=C
							role=terminal
							ha:spice/pinnum = { value=2; prio=31050; }
						}
					}
					ha:group.2 {
						uuid=4/pORh4XHs+cI/veJoAAAABA; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAS;
						x=-16000; y=0; mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=A
							role=terminal
							ha:spice/pinnum = { value=1; prio=31050; }
						}
					}
					ha:line.3 { x1=-4000; y1=0; x2=-6000; y2=0; stroke=sym-decor; }
					ha:line.4 { x1=-12000; y1=0; x2=-10000; y2=0; stroke=sym-decor; }
					ha:line.5 { x1=-10000; y1=4000; x2=-6000; y2=0; stroke=sym-decor; }
					ha:line.6 { x1=-6000; y1=0; x2=-10000; y2=-4000; stroke=sym-decor; }
					ha:line.7 { x1=-10000; y1=4000; x2=-10000; y2=-4000; stroke=sym-decor; }
					ha:line.8 { x1=-6000; y1=4000; x2=-6000; y2=-4000; stroke=sym-decor; }
					ha:text.9 { x1=-4000; y1=13000; rot=90.000000; dyntext=1; stroke=sym-secondary; text=%../a.devmap%; floater=1; }
					ha:text.10 { x1=-8000; y1=13000; rot=90.000000; dyntext=1; stroke=sym-primary; text=%../A.name%; floater=1; }
					ha:line.11 { x1=-8000; y1=8000; x2=-6000; y2=11000; stroke=sym-decor; }
					ha:line.12 { x1=-6000; y1=11000; x2=-7000; y2=10000; stroke=sym-decor; }
					ha:line.13 { x1=-6000; y1=11000; x2=-6517; y2=9545; stroke=sym-decor; }
					ha:line.14 { x1=-10000; y1=7000; x2=-8000; y2=10000; stroke=sym-decor; }
					ha:line.15 { x1=-8000; y1=10000; x2=-8000; y2=8000; stroke=sym-decor; }
					ha:line.16 { x1=-8303; y1=6354; x2=-6303; y2=9354; stroke=sym-decor; }
					ha:line.17 { x1=-6303; y1=9354; x2=-7303; y2=8354; stroke=sym-decor; }
					ha:line.18 { x1=-6303; y1=9354; x2=-6820; y2=7899; stroke=sym-decor; }
					ha:line.19 { x1=-10303; y1=5354; x2=-8303; y2=8354; stroke=sym-decor; }
					ha:line.20 { x1=-8303; y1=8354; x2=-8303; y2=6354; stroke=sym-decor; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					devmap=3mmLED_pth
					name=LED2
					role=symbol
					ha:spice/prefix = { value=D; prio=31050; }
				}
			}
			ha:group.8 {
				uuid=4/pORh4XHs+cI/veJoAAAABB; src_uuid=iNOQfJpO6hT/HFDFGjoAAABC;
				x=144000; y=104000; rot=270.000000; miry=1;
				li:objects {
					ha:group.1 {
						uuid=4/pORh4XHs+cI/veJoAAAABC; src_uuid=iNOQfJpO6hT/HFDFGjoAAABD;
						x=20000; y=0;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=2
							role=terminal
						}
					}
					ha:group.2 {
						uuid=4/pORh4XHs+cI/veJoAAAABD; src_uuid=iNOQfJpO6hT/HFDFGjoAAABE;
						mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=1
							role=terminal
						}
					}
					ha:text.3 { x1=12000; y1=2000; rot=90.000000; dyntext=1; stroke=sym-primary; text=%../a.value%; floater=1; }
					ha:text.4 { x1=8000; y1=2000; rot=90.000000; dyntext=1; stroke=sym-primary; text=%../A.name%; floater=1; }
					ha:polygon.5 {
						li:outline {
							ha:line { x1=4000; y1=2000; x2=4000; y2=-2000; }
							ha:line { x1=4000; y1=-2000; x2=16000; y2=-2000; }
							ha:line { x1=16000; y1=-2000; x2=16000; y2=2000; }
							ha:line { x1=16000; y1=2000; x2=4000; y2=2000; }
						}
						stroke=sym-decor;
					}
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					ha:device = { value=resistor; prio=31050; }
					footprint=acy(300)
					name=R2
					role=symbol
					ha:spice/prefix = { value=R; prio=31050; }
					value=220R
				}
			}
			ha:group.9 {
				uuid=4/pORh4XHs+cI/veJoAAAABE; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAQ;
				x=168000; y=84000; rot=270.000000;
				li:objects {
					ha:group.1 {
						uuid=4/pORh4XHs+cI/veJoAAAABF; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAR;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=C
							role=terminal
							ha:spice/pinnum = { value=2; prio=31050; }
						}
					}
					ha:group.2 {
						uuid=4/pORh4XHs+cI/veJoAAAABG; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAS;
						x=-16000; y=0; mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=A
							role=terminal
							ha:spice/pinnum = { value=1; prio=31050; }
						}
					}
					ha:line.3 { x1=-4000; y1=0; x2=-6000; y2=0; stroke=sym-decor; }
					ha:line.4 { x1=-12000; y1=0; x2=-10000; y2=0; stroke=sym-decor; }
					ha:line.5 { x1=-10000; y1=4000; x2=-6000; y2=0; stroke=sym-decor; }
					ha:line.6 { x1=-6000; y1=0; x2=-10000; y2=-4000; stroke=sym-decor; }
					ha:line.7 { x1=-10000; y1=4000; x2=-10000; y2=-4000; stroke=sym-decor; }
					ha:line.8 { x1=-6000; y1=4000; x2=-6000; y2=-4000; stroke=sym-decor; }
					ha:text.9 { x1=-4000; y1=13000; rot=90.000000; dyntext=1; stroke=sym-secondary; text=%../a.devmap%; floater=1; }
					ha:text.10 { x1=-8000; y1=13000; rot=90.000000; dyntext=1; stroke=sym-primary; text=%../A.name%; floater=1; }
					ha:line.11 { x1=-8000; y1=8000; x2=-6000; y2=11000; stroke=sym-decor; }
					ha:line.12 { x1=-6000; y1=11000; x2=-7000; y2=10000; stroke=sym-decor; }
					ha:line.13 { x1=-6000; y1=11000; x2=-6517; y2=9545; stroke=sym-decor; }
					ha:line.14 { x1=-10000; y1=7000; x2=-8000; y2=10000; stroke=sym-decor; }
					ha:line.15 { x1=-8000; y1=10000; x2=-8000; y2=8000; stroke=sym-decor; }
					ha:line.16 { x1=-8303; y1=6354; x2=-6303; y2=9354; stroke=sym-decor; }
					ha:line.17 { x1=-6303; y1=9354; x2=-7303; y2=8354; stroke=sym-decor; }
					ha:line.18 { x1=-6303; y1=9354; x2=-6820; y2=7899; stroke=sym-decor; }
					ha:line.19 { x1=-10303; y1=5354; x2=-8303; y2=8354; stroke=sym-decor; }
					ha:line.20 { x1=-8303; y1=8354; x2=-8303; y2=6354; stroke=sym-decor; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					devmap=3mmLED_pth
					name=LED3
					role=symbol
					ha:spice/prefix = { value=D; prio=31050; }
				}
			}
			ha:group.10 {
				uuid=4/pORh4XHs+cI/veJoAAAABH; src_uuid=iNOQfJpO6hT/HFDFGjoAAABC;
				x=168000; y=104000; rot=270.000000; miry=1;
				li:objects {
					ha:group.1 {
						uuid=4/pORh4XHs+cI/veJoAAAABI; src_uuid=iNOQfJpO6hT/HFDFGjoAAABD;
						x=20000; y=0;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=2
							role=terminal
						}
					}
					ha:group.2 {
						uuid=4/pORh4XHs+cI/veJoAAAABJ; src_uuid=iNOQfJpO6hT/HFDFGjoAAABE;
						mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=1
							role=terminal
						}
					}
					ha:text.3 { x1=12000; y1=2000; rot=90.000000; dyntext=1; stroke=sym-primary; text=%../a.value%; floater=1; }
					ha:text.4 { x1=8000; y1=2000; rot=90.000000; dyntext=1; stroke=sym-primary; text=%../A.name%; floater=1; }
					ha:polygon.5 {
						li:outline {
							ha:line { x1=4000; y1=2000; x2=4000; y2=-2000; }
							ha:line { x1=4000; y1=-2000; x2=16000; y2=-2000; }
							ha:line { x1=16000; y1=-2000; x2=16000; y2=2000; }
							ha:line { x1=16000; y1=2000; x2=4000; y2=2000; }
						}
						stroke=sym-decor;
					}
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					ha:device = { value=resistor; prio=31050; }
					footprint=acy(300)
					name=R3
					role=symbol
					ha:spice/prefix = { value=R; prio=31050; }
					value=220R
				}
			}
			ha:group.11 {
				uuid=4/pORh4XHs+cI/veJoAAAABK; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAQ;
				x=192000; y=84000; rot=270.000000;
				li:objects {
					ha:group.1 {
						uuid=4/pORh4XHs+cI/veJoAAAABL; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAR;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=C
							role=terminal
							ha:spice/pinnum = { value=2; prio=31050; }
						}
					}
					ha:group.2 {
						uuid=4/pORh4XHs+cI/veJoAAAABM; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAS;
						x=-16000; y=0; mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=A
							role=terminal
							ha:spice/pinnum = { value=1; prio=31050; }
						}
					}
					ha:line.3 { x1=-4000; y1=0; x2=-6000; y2=0; stroke=sym-decor; }
					ha:line.4 { x1=-12000; y1=0; x2=-10000; y2=0; stroke=sym-decor; }
					ha:line.5 { x1=-10000; y1=4000; x2=-6000; y2=0; stroke=sym-decor; }
					ha:line.6 { x1=-6000; y1=0; x2=-10000; y2=-4000; stroke=sym-decor; }
					ha:line.7 { x1=-10000; y1=4000; x2=-10000; y2=-4000; stroke=sym-decor; }
					ha:line.8 { x1=-6000; y1=4000; x2=-6000; y2=-4000; stroke=sym-decor; }
					ha:text.9 { x1=-4000; y1=13000; rot=90.000000; dyntext=1; stroke=sym-secondary; text=%../a.devmap%; floater=1; }
					ha:text.10 { x1=-8000; y1=13000; rot=90.000000; dyntext=1; stroke=sym-primary; text=%../A.name%; floater=1; }
					ha:line.11 { x1=-8000; y1=8000; x2=-6000; y2=11000; stroke=sym-decor; }
					ha:line.12 { x1=-6000; y1=11000; x2=-7000; y2=10000; stroke=sym-decor; }
					ha:line.13 { x1=-6000; y1=11000; x2=-6517; y2=9545; stroke=sym-decor; }
					ha:line.14 { x1=-10000; y1=7000; x2=-8000; y2=10000; stroke=sym-decor; }
					ha:line.15 { x1=-8000; y1=10000; x2=-8000; y2=8000; stroke=sym-decor; }
					ha:line.16 { x1=-8303; y1=6354; x2=-6303; y2=9354; stroke=sym-decor; }
					ha:line.17 { x1=-6303; y1=9354; x2=-7303; y2=8354; stroke=sym-decor; }
					ha:line.18 { x1=-6303; y1=9354; x2=-6820; y2=7899; stroke=sym-decor; }
					ha:line.19 { x1=-10303; y1=5354; x2=-8303; y2=8354; stroke=sym-decor; }
					ha:line.20 { x1=-8303; y1=8354; x2=-8303; y2=6354; stroke=sym-decor; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					devmap=3mmLED_pth
					name=LED4
					role=symbol
					ha:spice/prefix = { value=D; prio=31050; }
				}
			}
			ha:group.12 {
				uuid=4/pORh4XHs+cI/veJoAAAABN; src_uuid=iNOQfJpO6hT/HFDFGjoAAABC;
				x=192000; y=104000; rot=270.000000; miry=1;
				li:objects {
					ha:group.1 {
						uuid=4/pORh4XHs+cI/veJoAAAABO; src_uuid=iNOQfJpO6hT/HFDFGjoAAABD;
						x=20000; y=0;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=2
							role=terminal
						}
					}
					ha:group.2 {
						uuid=4/pORh4XHs+cI/veJoAAAABP; src_uuid=iNOQfJpO6hT/HFDFGjoAAABE;
						mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=1
							role=terminal
						}
					}
					ha:text.3 { x1=12000; y1=2000; rot=90.000000; dyntext=1; stroke=sym-primary; text=%../a.value%; floater=1; }
					ha:text.4 { x1=8000; y1=2000; rot=90.000000; dyntext=1; stroke=sym-primary; text=%../A.name%; floater=1; }
					ha:polygon.5 {
						li:outline {
							ha:line { x1=4000; y1=2000; x2=4000; y2=-2000; }
							ha:line { x1=4000; y1=-2000; x2=16000; y2=-2000; }
							ha:line { x1=16000; y1=-2000; x2=16000; y2=2000; }
							ha:line { x1=16000; y1=2000; x2=4000; y2=2000; }
						}
						stroke=sym-decor;
					}
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					ha:device = { value=resistor; prio=31050; }
					footprint=acy(300)
					name=R4
					role=symbol
					ha:spice/prefix = { value=R; prio=31050; }
					value=220R
				}
			}
			ha:group.14 {
				uuid=4/pORh4XHs+cI/veJoAAAABc; src_uuid=3s0ePx27Ce5+YP4+xH0AAADp;
				x=216000; y=108000; rot=270.000000;
				li:objects {
					ha:arc.1 { cx=6000; cy=9000; r=6000; sang=0.000000; dang=-180.000000; stroke=sym-decor; }
					ha:group.2 {
						uuid=4/pORh4XHs+cI/veJoAAAABd; src_uuid=iNOQfJpO6hT/HFDFGjoAAAB4;
						x=12000; y=0;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=2
							role=terminal
							ha:spice/pinnum = { value=2; prio=31050; }
						}
					}
					ha:group.3 {
						uuid=4/pORh4XHs+cI/veJoAAAABe; src_uuid=iNOQfJpO6hT/HFDFGjoAAAB5;
						mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=1
							role=terminal
							ha:spice/pinnum = { value=1; prio=31050; }
						}
					}
					ha:line.4 { x1=0; y1=9000; x2=12000; y2=9000; stroke=sym-decor; }
					ha:line.5 { x1=4000; y1=0; x2=4000; y2=3343; stroke=sym-decor; }
					ha:line.6 { x1=8000; y1=0; x2=8000; y2=3343; stroke=sym-decor; }
					ha:text.7 { x1=0; y1=9000; dyntext=1; stroke=sym-primary; text=%../A.name%; floater=1; }
				}
				ha:attrib {
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					footprint=piezo_5mm_to_7.62mm.rf
					name=PIEZO
					role=symbol
				}
			}
			ha:group.16 {
				uuid=4/pORh4XHs+cI/veJoAAAADG; src_uuid=4/pORh4XHs+cI/veJoAAAADD;
				x=172000; y=176000;
				li:objects {
					ha:text.1 { x1=0; y1=6000; dyntext=1; stroke=sym-primary; text=%../A.name%; floater=1; }
					ha:group.2 {
						uuid=4/pORh4XHs+cI/veJoAAAADH; src_uuid=4/pORh4XHs+cI/veJoAAAADE;
						mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../A.name%; }
						}
						ha:attrib {
							name=1
							role=terminal
						}
					}
					ha:group.3 {
						uuid=4/pORh4XHs+cI/veJoAAAADI; src_uuid=4/pORh4XHs+cI/veJoAAAADF;
						x=0; y=4000; mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../A.name%; }
						}
						ha:attrib {
							name=2
							role=terminal
						}
					}
					ha:polygon.4 {
						li:outline {
							ha:line { x1=0; y1=-2000; x2=0; y2=6000; }
							ha:line { x1=0; y1=6000; x2=4000; y2=6000; }
							ha:line { x1=4000; y1=6000; x2=4000; y2=-2000; }
							ha:line { x1=4000; y1=-2000; x2=0; y2=-2000; }
						}
						stroke=sym-decor;
					}
				}
				ha:attrib {
					footprint=connector(1,2)
					name=5V
					role=symbol
					spice/omit=yes
				}
			}
			ha:group.17 {
				uuid=4/pORh4XHs+cI/veJoAAAADX; src_uuid=4/pORh4XHs+cI/veJoAAAADQ;
				x=132000; y=180000; mirx=1; miry=1;
				li:objects {
					ha:text.1 { x1=0; y1=-6000; dyntext=1; stroke=sym-primary; text=%../A.name%; floater=1; }
					ha:group.2 {
						uuid=4/pORh4XHs+cI/veJoAAAADY; src_uuid=4/pORh4XHs+cI/veJoAAAADR;
						mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../A.name%; }
						}
						ha:attrib {
							name=1
							role=terminal
						}
					}
					ha:group.3 {
						uuid=4/pORh4XHs+cI/veJoAAAADZ; src_uuid=4/pORh4XHs+cI/veJoAAAADS;
						x=0; y=4000; mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../A.name%; }
						}
						ha:attrib {
							name=2
							role=terminal
						}
					}
					ha:group.4 {
						uuid=4/pORh4XHs+cI/veJoAAAADa; src_uuid=4/pORh4XHs+cI/veJoAAAADT;
						x=0; y=8000; mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../A.name%; }
						}
						ha:attrib {
							name=3
							role=terminal
						}
					}
					ha:group.5 {
						uuid=4/pORh4XHs+cI/veJoAAAADb; src_uuid=4/pORh4XHs+cI/veJoAAAADU;
						x=0; y=12000; mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../A.name%; }
						}
						ha:attrib {
							name=4
							role=terminal
						}
					}
					ha:group.6 {
						uuid=4/pORh4XHs+cI/veJoAAAADc; src_uuid=4/pORh4XHs+cI/veJoAAAADV;
						x=0; y=16000; mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../A.name%; }
						}
						ha:attrib {
							name=5
							role=terminal
						}
					}
					ha:group.7 {
						uuid=4/pORh4XHs+cI/veJoAAAADd; src_uuid=4/pORh4XHs+cI/veJoAAAADW;
						x=0; y=20000; mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../A.name%; }
						}
						ha:attrib {
							name=6
							role=terminal
						}
					}
					ha:polygon.8 {
						li:outline {
							ha:line { x1=0; y1=-2000; x2=0; y2=22000; }
							ha:line { x1=0; y1=22000; x2=4000; y2=22000; }
							ha:line { x1=4000; y1=22000; x2=4000; y2=-2000; }
							ha:line { x1=4000; y1=-2000; x2=0; y2=-2000; }
						}
						stroke=sym-decor;
					}
				}
				ha:attrib {
					footprint=USB_B_180_degree_PTH.rf
					name=USB1
					role=symbol
					spice/omit=yes
				}
			}
			ha:group.18 {
				uuid=4/pORh4XHs+cI/veJoAAAADk; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAh;
				x=152000; y=180000; rot=270.000000;
				li:objects {
					ha:group.1 {
						uuid=4/pORh4XHs+cI/veJoAAAADl; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAi;
						x=20000; y=0;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=2
							role=terminal
						}
					}
					ha:group.2 {
						uuid=4/pORh4XHs+cI/veJoAAAADm; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAj;
						mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=1
							role=terminal
						}
					}
					ha:text.3 { x1=12000; y1=6000; rot=90.000000; dyntext=1; stroke=sym-primary; text=%../a.value%; floater=1; }
					ha:text.4 { x1=8000; y1=6000; rot=90.000000; dyntext=1; stroke=sym-primary; text=%../A.name%; floater=1; }
					ha:line.5 { x1=9000; y1=5000; x2=9000; y2=-5000; stroke=sym-decor; }
					ha:line.6 { x1=11000; y1=5000; x2=11000; y2=-5000; stroke=sym-decor; }
					ha:line.7 { x1=4000; y1=0; x2=9000; y2=0; stroke=sym-decor; }
					ha:line.8 { x1=11000; y1=0; x2=16000; y2=0; stroke=sym-decor; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					ha:device = { value=capacitor; prio=31050; }
					footprint=C200.rf
					name=C1
					role=symbol
					ha:spice/prefix = { value=C; prio=31050; }
					value=<n/a>
				}
			}
			ha:group.19 {
				uuid=4/pORh4XHs+cI/veJoAAAADs; src_uuid=iNOQfJpO6hT/HFDFGjoAAABv;
				x=140000; y=184000;
				li:objects {
					ha:group.1 {
						uuid=4/pORh4XHs+cI/veJoAAAADt; src_uuid=iNOQfJpO6hT/HFDFGjoAAABw;
						rot=270.000000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
						}
						ha:attrib {
							ha:name = { value=1; prio=220; }
							role=terminal
						}
					}
					ha:line.2 { x1=2500; y1=4000; x2=-2500; y2=4000; stroke=sym-decor; }
					ha:text.3 { x1=-6000; y1=4000; x2=6000; y2=7000; halign=center; dyntext=1; stroke=sym-primary; text=%../A.rail%; floater=1; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					li:forge {
						delete,forge/tmp
						scalar,forge/tmp
						{sub,^,1:,forge/tmp}
						suba,$,rail,forge/tmp
						array,connect
						append,connect,forge/tmp
					}
					rail=5V
					role=symbol
				}
			}
			ha:group.20 {
				uuid=4/pORh4XHs+cI/veJoAAAADy; src_uuid=iNOQfJpO6hT/HFDFGjoAAABm;
				x=140000; y=156000;
				li:objects {
					ha:group.1 {
						uuid=4/pORh4XHs+cI/veJoAAAADz; src_uuid=iNOQfJpO6hT/HFDFGjoAAABn;
						rot=90.000000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
						}
						ha:attrib {
							ha:name = { value=1; prio=220; }
							role=terminal
						}
					}
					ha:line.2 { x1=-1500; y1=-5000; x2=1500; y2=-5000; stroke=sym-decor; }
					ha:line.3 { x1=-500; y1=-6000; x2=500; y2=-6000; stroke=sym-decor; }
					ha:line.4 { x1=-2500; y1=-4000; x2=2500; y2=-4000; stroke=sym-decor; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					li:connect {
						{1:GND}
					}
					role=symbol
				}
			}
			ha:group.21 {
				uuid=4/pORh4XHs+cI/veJoAAAAD0;
				li:objects {
					ha:line.2 { x1=140000; y1=180000; x2=140000; y2=184000; stroke=wire; }
					ha:line.4 { x1=140000; y1=180000; x2=140000; y2=180000; stroke=junction; }
					ha:line.5 { x1=136000; y1=180000; x2=168000; y2=180000; stroke=wire; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:connection.22 {
				li:conn {
					/2/17/2/1
					/2/21/5
				}
			}
			ha:connection.23 {
				li:conn {
					/2/21/2
					/2/19/1/1
				}
			}
			ha:connection.24 {
				li:conn {
					/2/18/2/1
					/2/21/5
				}
			}
			ha:group.26 {
				uuid=4/pORh4XHs+cI/veJoAAAAD1;
				li:objects {
					ha:line.1 { x1=164000; y1=176000; x2=164000; y2=160000; stroke=wire; }
					ha:line.3 { x1=136000; y1=160000; x2=164000; y2=160000; stroke=wire; }
					ha:line.5 { x1=140000; y1=160000; x2=140000; y2=160000; stroke=junction; }
					ha:line.7 { x1=140000; y1=164000; x2=136000; y2=164000; stroke=wire; }
					ha:line.8 { x1=140000; y1=156000; x2=140000; y2=168000; stroke=wire; }
					ha:line.9 { x1=140000; y1=164000; x2=140000; y2=164000; stroke=junction; }
					ha:line.10 { x1=140000; y1=168000; x2=136000; y2=168000; stroke=wire; }
					ha:line.11 { x1=164000; y1=176000; x2=168000; y2=176000; stroke=wire; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:connection.28 {
				li:conn {
					/2/18/1/1
					/2/26/3
				}
			}
			ha:connection.29 {
				li:conn {
					/2/26/3
					/2/17/7/1
				}
			}
			ha:connection.30 {
				li:conn {
					/2/20/1/1
					/2/26/8
				}
			}
			ha:connection.31 {
				li:conn {
					/2/26/7
					/2/17/6/1
				}
			}
			ha:connection.32 {
				li:conn {
					/2/26/10
					/2/17/5/1
				}
			}
			ha:connection.33 {
				li:conn {
					/2/21/5
					/2/16/3/1
				}
			}
			ha:connection.34 {
				li:conn {
					/2/26/11
					/2/16/2/1
				}
			}
			ha:group.35 {
				uuid=4/pORh4XHs+cI/veJoAAAAD4; src_uuid=iNOQfJpO6hT/HFDFGjoAAABm;
				x=60000; y=76000;
				li:objects {
					ha:group.1 {
						uuid=4/pORh4XHs+cI/veJoAAAAD5; src_uuid=iNOQfJpO6hT/HFDFGjoAAABn;
						rot=90.000000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
						}
						ha:attrib {
							ha:name = { value=1; prio=220; }
							role=terminal
						}
					}
					ha:line.2 { x1=-1500; y1=-5000; x2=1500; y2=-5000; stroke=sym-decor; }
					ha:line.3 { x1=-500; y1=-6000; x2=500; y2=-6000; stroke=sym-decor; }
					ha:line.4 { x1=-2500; y1=-4000; x2=2500; y2=-4000; stroke=sym-decor; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					li:connect {
						{1:GND}
					}
					role=symbol
				}
			}
			ha:group.45 {
				uuid=4/pORh4XHs+cI/veJoAAAAD8;
				li:objects {
					ha:line.1 { x1=120000; y1=104000; x2=120000; y2=100000; stroke=wire; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:connection.47 {
				li:conn {
					/2/45/1
					/2/5/2/1
				}
			}
			ha:group.48 {
				uuid=4/pORh4XHs+cI/veJoAAAAD9;
				li:objects {
					ha:line.1 { x1=144000; y1=104000; x2=144000; y2=100000; stroke=wire; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:connection.49 {
				li:conn {
					/2/48/1
					/2/7/2/1
				}
			}
			ha:group.51 {
				uuid=4/pORh4XHs+cI/veJoAAAAD+;
				li:objects {
					ha:line.1 { x1=168000; y1=104000; x2=168000; y2=100000; stroke=wire; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:connection.52 {
				li:conn {
					/2/51/1
					/2/9/2/1
				}
			}
			ha:group.54 {
				uuid=4/pORh4XHs+cI/veJoAAAAD/;
				li:objects {
					ha:line.1 { x1=192000; y1=104000; x2=192000; y2=100000; stroke=wire; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:connection.55 {
				li:conn {
					/2/54/1
					/2/11/2/1
				}
			}
			ha:group.58 {
				uuid=4/pORh4XHs+cI/veJoAAAAEE;
				x=0; y=8000;
				li:objects {
					ha:line.1 { x1=60000; y1=68000; x2=60000; y2=108000; stroke=wire; }
					ha:line.2 { x1=216000; y1=72000; x2=216000; y2=88000; stroke=wire; }
					ha:line.3 { x1=60000; y1=72000; x2=216000; y2=72000; stroke=wire; }
					ha:line.4 { x1=120000; y1=72000; x2=120000; y2=72000; stroke=junction; }
					ha:line.5 { x1=192000; y1=72000; x2=192000; y2=72000; stroke=junction; }
					ha:line.6 { x1=192000; y1=72000; x2=192000; y2=76000; stroke=wire; }
					ha:line.7 { x1=168000; y1=72000; x2=168000; y2=72000; stroke=junction; }
					ha:line.8 { x1=144000; y1=72000; x2=144000; y2=76000; stroke=wire; }
					ha:line.9 { x1=120000; y1=76000; x2=120000; y2=72000; stroke=wire; }
					ha:line.10 { x1=144000; y1=72000; x2=144000; y2=72000; stroke=junction; }
					ha:line.11 { x1=168000; y1=72000; x2=168000; y2=76000; stroke=wire; }
					ha:line.12 { x1=60000; y1=72000; x2=60000; y2=72000; stroke=junction; }
					ha:line.13 { x1=76000; y1=72000; x2=76000; y2=74000; stroke=wire; }
					ha:line.14 { x1=76000; y1=72000; x2=76000; y2=72000; stroke=junction; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:group.61 {
				uuid=4/pORh4XHs+cI/veJoAAAAEI; src_uuid=iNOQfJpO6hT/HFDFGjoAAABv;
				x=60000; y=156000;
				li:objects {
					ha:group.1 {
						uuid=4/pORh4XHs+cI/veJoAAAAEJ; src_uuid=iNOQfJpO6hT/HFDFGjoAAABw;
						rot=270.000000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
						}
						ha:attrib {
							ha:name = { value=1; prio=220; }
							role=terminal
						}
					}
					ha:line.2 { x1=2500; y1=4000; x2=-2500; y2=4000; stroke=sym-decor; }
					ha:text.3 { x1=-6000; y1=4000; x2=6000; y2=7000; halign=center; dyntext=1; stroke=sym-primary; text=%../A.rail%; floater=1; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					li:forge {
						delete,forge/tmp
						scalar,forge/tmp
						{sub,^,1:,forge/tmp}
						suba,$,rail,forge/tmp
						array,connect
						append,connect,forge/tmp
					}
					rail=5V
					role=symbol
				}
			}
			ha:group.62 {
				uuid=4/pORh4XHs+cI/veJoAAAAEK; src_uuid=4/pORh4XHs+cI/veJoAAAAEH;
				x=60000; y=152000;
				li:objects {
					ha:line.1 { x1=0; y1=0; x2=0; y2=4000; stroke=wire; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:connection.63 {
				li:conn {
					/2/62/1
					/2/2/9/1
				}
			}
			ha:connection.67 {
				li:conn {
					/2/58/1
					/2/2/10/1
				}
			}
			ha:connection.68 {
				li:conn {
					/2/62/1
					/2/61/1/1
				}
			}
			ha:group.69 {
				uuid=4/pORh4XHs+cI/veJoAAAAEL;
				li:objects {
					ha:line.1 { x1=72000; y1=132000; x2=120000; y2=132000; stroke=wire; }
					ha:line.3 { x1=120000; y1=132000; x2=120000; y2=124000; stroke=wire; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:group.72 {
				uuid=4/pORh4XHs+cI/veJoAAAAEM;
				li:objects {
					ha:line.1 { x1=72000; y1=128000; x2=144000; y2=128000; stroke=wire; }
					ha:line.2 { x1=144000; y1=128000; x2=144000; y2=124000; stroke=wire; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:group.75 {
				uuid=4/pORh4XHs+cI/veJoAAAAEN;
				li:objects {
					ha:line.1 { x1=72000; y1=136000; x2=168000; y2=136000; stroke=wire; }
					ha:line.2 { x1=168000; y1=136000; x2=168000; y2=124000; stroke=wire; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:connection.76 {
				li:conn {
					/2/75/1
					/2/2/5/1
				}
			}
			ha:group.78 {
				uuid=4/pORh4XHs+cI/veJoAAAAEO;
				li:objects {
					ha:line.1 { x1=72000; y1=144000; x2=192000; y2=144000; stroke=wire; }
					ha:line.5 { x1=192000; y1=144000; x2=192000; y2=124000; stroke=wire; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:group.84 {
				uuid=4/pORh4XHs+cI/veJoAAAAEX; src_uuid=iNOQfJpO6hT/HFDFGjoAAABC;
				x=76000; y=102000; rot=270.000000;
				li:objects {
					ha:group.1 {
						uuid=4/pORh4XHs+cI/veJoAAAAEY; src_uuid=iNOQfJpO6hT/HFDFGjoAAABD;
						x=20000; y=0;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=2
							role=terminal
						}
					}
					ha:group.2 {
						uuid=4/pORh4XHs+cI/veJoAAAAEZ; src_uuid=iNOQfJpO6hT/HFDFGjoAAABE;
						mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=1
							role=terminal
						}
					}
					ha:text.3 { x1=12000; y1=2000; rot=90.000000; dyntext=1; stroke=sym-primary; text=%../a.value%; floater=1; }
					ha:text.4 { x1=8000; y1=2000; rot=90.000000; dyntext=1; stroke=sym-primary; text=%../A.name%; floater=1; }
					ha:polygon.5 {
						li:outline {
							ha:line { x1=4000; y1=2000; x2=4000; y2=-2000; }
							ha:line { x1=4000; y1=-2000; x2=16000; y2=-2000; }
							ha:line { x1=16000; y1=-2000; x2=16000; y2=2000; }
							ha:line { x1=16000; y1=2000; x2=4000; y2=2000; }
						}
						stroke=sym-decor;
					}
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					ha:device = { value=resistor; prio=31050; }
					footprint=acy(300)
					name=R5
					role=symbol
					ha:spice/prefix = { value=R; prio=31050; }
					value=47k
				}
			}
			ha:group.88 {
				uuid=4/pORh4XHs+cI/veJoAAAAEe; src_uuid=iNOQfJpO6hT/HFDFGjoAAABC;
				x=88000; y=124000;
				li:objects {
					ha:group.1 {
						uuid=4/pORh4XHs+cI/veJoAAAAEf; src_uuid=iNOQfJpO6hT/HFDFGjoAAABD;
						x=20000; y=0;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=2
							role=terminal
						}
					}
					ha:group.2 {
						uuid=4/pORh4XHs+cI/veJoAAAAEg; src_uuid=iNOQfJpO6hT/HFDFGjoAAABE;
						mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=1
							role=terminal
						}
					}
					ha:text.3 { x1=5000; y1=-8000; dyntext=1; stroke=sym-primary; text=%../a.value%; floater=1; }
					ha:text.4 { x1=5000; y1=-5000; dyntext=1; stroke=sym-primary; text=%../A.name%; floater=1; }
					ha:polygon.5 {
						li:outline {
							ha:line { x1=4000; y1=2000; x2=4000; y2=-2000; }
							ha:line { x1=4000; y1=-2000; x2=16000; y2=-2000; }
							ha:line { x1=16000; y1=-2000; x2=16000; y2=2000; }
							ha:line { x1=16000; y1=2000; x2=4000; y2=2000; }
						}
						stroke=sym-decor;
					}
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					ha:device = { value=resistor; prio=31050; }
					footprint=acy(300)
					name=R6
					role=symbol
					ha:spice/prefix = { value=R; prio=31050; }
					value=10k
				}
			}
			ha:group.91 {
				uuid=4/pORh4XHs+cI/veJoAAAAEh;
				x=-28000; y=0;
				li:objects {
					ha:line.1 { x1=104000; y1=102000; x2=104000; y2=106000; stroke=wire; }
					ha:line.2 { x1=104000; y1=104000; x2=112000; y2=104000; stroke=wire; }
					ha:line.3 { x1=104000; y1=104000; x2=104000; y2=104000; stroke=junction; }
					ha:line.4 { x1=112000; y1=104000; x2=112000; y2=106000; stroke=wire; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:group.94 {
				uuid=4/pORh4XHs+cI/veJoAAAAEi;
				li:objects {
					ha:line.3 { x1=76000; y1=124000; x2=76000; y2=122000; stroke=wire; }
					ha:line.4 { x1=76000; y1=124000; x2=76000; y2=124000; stroke=junction; }
					ha:line.5 { x1=72000; y1=124000; x2=88000; y2=124000; stroke=wire; }
					ha:line.6 { x1=84000; y1=122000; x2=84000; y2=124000; stroke=wire; }
					ha:line.7 { x1=84000; y1=124000; x2=84000; y2=124000; stroke=junction; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:connection.115 {
				li:conn {
					/2/91/1
					/2/84/2/1
				}
			}
			ha:connection.117 {
				li:conn {
					/2/2/8/1
					/2/94/5
				}
			}
			ha:group.120 {
				uuid=4/pORh4XHs+cI/veJoAAAAEk;
				x=8000; y=0;
				li:objects {
					ha:line.1 { x1=100000; y1=124000; x2=104000; y2=124000; stroke=wire; }
					ha:line.16 { x1=104000; y1=124000; x2=104000; y2=156000; stroke=wire; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:connection.121 {
				li:conn {
					/2/120/1
					/2/88/1/1
				}
			}
			ha:connection.125 {
				li:conn {
					/2/14/2/1
					/2/58/2
				}
			}
			ha:connection.126 {
				li:conn {
					/2/11/1/1
					/2/58/6
				}
			}
			ha:connection.127 {
				li:conn {
					/2/7/1/1
					/2/58/8
				}
			}
			ha:connection.128 {
				li:conn {
					/2/5/1/1
					/2/58/9
				}
			}
			ha:connection.129 {
				li:conn {
					/2/9/1/1
					/2/58/11
				}
			}
			ha:group.130 {
				uuid=4/pORh4XHs+cI/veJoAAAAEo; src_uuid=iNOQfJpO6hT/HFDFGjoAAABv;
				x=112000; y=156000;
				li:objects {
					ha:group.1 {
						uuid=4/pORh4XHs+cI/veJoAAAAEp; src_uuid=iNOQfJpO6hT/HFDFGjoAAABw;
						rot=270.000000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
						}
						ha:attrib {
							ha:name = { value=1; prio=220; }
							role=terminal
						}
					}
					ha:line.2 { x1=2500; y1=4000; x2=-2500; y2=4000; stroke=sym-decor; }
					ha:text.3 { x1=-6000; y1=4000; x2=6000; y2=7000; halign=center; dyntext=1; stroke=sym-primary; text=%../A.rail%; floater=1; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					li:forge {
						delete,forge/tmp
						scalar,forge/tmp
						{sub,^,1:,forge/tmp}
						suba,$,rail,forge/tmp
						array,connect
						append,connect,forge/tmp
					}
					rail=5V
					role=symbol
				}
			}
			ha:group.133 {
				uuid=4/pORh4XHs+cI/veJoAAAAEq;
				li:objects {
					ha:line.1 { x1=216000; y1=108000; x2=216000; y2=140000; stroke=wire; }
					ha:line.2 { x1=68000; y1=140000; x2=216000; y2=140000; stroke=wire; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:connection.134 {
				li:conn {
					/2/133/1
					/2/14/3/1
				}
			}
			ha:connection.135 {
				li:conn {
					/2/58/1
					/2/35/1/1
				}
			}
			ha:connection.138 {
				li:conn {
					/2/58/13
					/2/84/1/1
				}
			}
			ha:connection.148 {
				li:conn {
					/2/6/2/1
					/2/45/1
				}
			}
			ha:connection.149 {
				li:conn {
					/2/8/1/1
					/2/72/2
				}
			}
			ha:connection.150 {
				li:conn {
					/2/8/2/1
					/2/48/1
				}
			}
			ha:connection.151 {
				li:conn {
					/2/10/1/1
					/2/75/2
				}
			}
			ha:connection.152 {
				li:conn {
					/2/10/2/1
					/2/51/1
				}
			}
			ha:connection.154 {
				li:conn {
					/2/12/2/1
					/2/54/1
				}
			}
			ha:connection.155 {
				li:conn {
					/2/69/1
					/2/2/6/1
				}
			}
			ha:connection.157 {
				li:conn {
					/2/6/1/1
					/2/69/3
				}
			}
			ha:connection.158 {
				li:conn {
					/2/72/1
					/2/2/7/1
				}
			}
			ha:connection.161 {
				li:conn {
					/2/133/2
					/2/2/4/1
				}
			}
			ha:connection.162 {
				li:conn {
					/2/78/1
					/2/2/3/1
				}
			}
			ha:connection.164 {
				li:conn {
					/2/12/1/1
					/2/78/5
				}
			}
			ha:connection.167 {
				li:conn {
					/2/130/1/1
					/2/120/16
				}
			}
			ha:group.168 {
				uuid=4/pORh4XHs+cI/veJoAAAAGW; src_uuid=4/pORh4XHs+cI/veJoAAAAGR;
				x=76000; y=110000; rot=270.000000; miry=1;
				li:objects {
					ha:text.1 { x1=0; y1=-5000; dyntext=1; stroke=sym-primary; text=%../A.name%; floater=1; }
					ha:group.2 {
						uuid=4/pORh4XHs+cI/veJoAAAAGX; src_uuid=4/pORh4XHs+cI/veJoAAAAGS;
						mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../A.name%; }
						}
						ha:attrib {
							name=1
							role=terminal
						}
					}
					ha:line.3 { x1=0; y1=0; x2=1200; y2=0; stroke=sym-decor; }
					ha:group.4 {
						uuid=4/pORh4XHs+cI/veJoAAAAGY; src_uuid=4/pORh4XHs+cI/veJoAAAAGT;
						x=8000; y=0;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../A.name%; }
						}
						ha:attrib {
							name=3
							role=terminal
						}
					}
					ha:line.5 { x1=6800; y1=0; x2=8000; y2=0; stroke=sym-decor; }
					ha:arc.6 { cx=6400; cy=0; r=400; sang=0.000000; dang=360.000000; stroke=sym-decor; }
					ha:line.7 { x1=1200; y1=0; x2=5600; y2=3200; stroke=sym-decor; }
					ha:group.8 {
						uuid=4/pORh4XHs+cI/veJoAAAAGZ; src_uuid=4/pORh4XHs+cI/veJoAAAAGU;
						x=0; y=8000; mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../A.name%; }
						}
						ha:attrib {
							name=2
							role=terminal
						}
					}
					ha:line.9 { x1=0; y1=8000; x2=1200; y2=8000; stroke=sym-decor; }
					ha:group.10 {
						uuid=4/pORh4XHs+cI/veJoAAAAGa; src_uuid=4/pORh4XHs+cI/veJoAAAAGV;
						x=8000; y=8000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../A.name%; }
						}
						ha:attrib {
							name=4
							role=terminal
						}
					}
					ha:line.11 { x1=6800; y1=8000; x2=8000; y2=8000; stroke=sym-decor; }
					ha:arc.12 { cx=6400; cy=8000; r=400; sang=0.000000; dang=360.000000; stroke=sym-decor; }
					ha:line.13 { x1=1200; y1=8000; x2=5600; y2=11200; stroke=sym-decor; }
					ha:line.14 { x1=4000; y1=2000; x2=4000; y2=6000; stroke=sym-decor; }
				}
				ha:attrib {
					footprint=TACT_6x6_4p
					name=LAND
					role=symbol
					spice/omit=yes
				}
			}
			ha:connection.169 {
				li:conn {
					/2/168/2/1
					/2/91/1
				}
			}
			ha:connection.170 {
				li:conn {
					/2/168/4/1
					/2/94/3
				}
			}
			ha:connection.171 {
				li:conn {
					/2/94/5
					/2/88/2/1
				}
			}
			ha:connection.172 {
				li:conn {
					/2/94/6
					/2/168/10/1
				}
			}
			ha:connection.173 {
				li:conn {
					/2/91/4
					/2/168/8/1
				}
			}
		}
		ha:attrib {
			drawing_min_height=200000
			drawing_min_width=287000
			maintainer=<maint. attr>
			page=<page attr>
			print_page=A/4
			title=<please set sheet title attribute>
		}
	}
  li:sch-rnd-conf-v1 {
   ha:overwrite {
    ha:editor {
     grids_idx = 0
     grid = 1.0240 mm
    }
   }
  }
}
