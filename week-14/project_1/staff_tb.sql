--
-- PostgreSQL database dump
--

\restrict OOm1NtUUrUpUTnapMctgaME9EvT74Bgc0dHe0qteRpEaYWcdNA9l6zPbwZoqCWh

-- Dumped from database version 18.6
-- Dumped by pg_dump version 18.6

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET transaction_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SELECT pg_catalog.set_config('search_path', '', false);
SET check_function_bodies = false;
SET xmloption = content;
SET client_min_messages = warning;
SET row_security = off;

SET default_tablespace = '';

SET default_table_access_method = heap;

--
-- Name: staff; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.staff (
    staff_id integer CONSTRAINT employees_eid_not_null NOT NULL,
    staff_name text CONSTRAINT employees_name_not_null NOT NULL,
    dno integer CONSTRAINT employees_dno_not_null NOT NULL,
    staff_sal real CONSTRAINT employees_esal_not_null NOT NULL,
    age integer CONSTRAINT employees_age_not_null NOT NULL,
    mobile character(20),
    gender character varying(1)
);


ALTER TABLE public.staff OWNER TO postgres;

--
-- Data for Name: staff; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.staff (staff_id, staff_name, dno, staff_sal, age, mobile, gender) FROM stdin;
101	Alade Joy	2	250000	33	08023089832         	\N
100	Mustapha Ali	3	175000	32	08063285831         	\N
107	Alokwe Martin	7	380000	48	07090082812         	\N
97	Dankade Aminat	5	550000	40	09023688832         	\N
108	Josiah Joshua	1	120000	30	08053189131         	\N
102	Makinde Mary	2	450000	55	09023487830         	\N
120	Adeleke Jane	4	200000	38	07061045862         	\N
122	Osahon Mark	6	320000	44	08097656453         	\N
117	Suleman Ajayi	3	800000	50	07030067894         	\N
104	Kuti Lawal	1	750000	35	09145689842         	\N
\.


--
-- PostgreSQL database dump complete
--

\unrestrict OOm1NtUUrUpUTnapMctgaME9EvT74Bgc0dHe0qteRpEaYWcdNA9l6zPbwZoqCWh

