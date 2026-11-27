--
-- PostgreSQL database dump
--

\restrict PyhwZYsdwfke3ZxFpUeBo4dzKYCgtpjINqEMHBJ6NHmhdQoaL3Bj1nf1T1a4uWK

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
-- Name: customer; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.customer (
    c_id integer NOT NULL,
    c_name text NOT NULL,
    c_age integer NOT NULL,
    c_email text NOT NULL,
    c_mobile character(20),
    eid integer NOT NULL,
    data_id integer NOT NULL
);


ALTER TABLE public.customer OWNER TO postgres;

--
-- Data for Name: customer; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.customer (c_id, c_name, c_age, c_email, c_mobile, eid, data_id) FROM stdin;
110	Musta Karim	35	m_karim@gmail.com	08055089112         	102	5
111	Lilian Jaiya	43	l_jaiye@gmail.com	08055185341         	100	3
112	Arthur Musa	50	a_musa@gmail.com	07055282813         	107	10
113	Philip Akonjo	41	p_akonjo@gmail.com	09052356772         	100	2
114	Marylen Mapa	33	m_mapa@gmail.com	08053333551         	120	5
115	Oghenero Agor	50	o_agor@gmail.com	07055566774         	117	11
116	Adams Bree	33	a_bree@gamil.com	08056765424         	102	1
117	Okafor Mathais	45	o_mathias@gmail.com	080567673367        	120	10
118	Samson Adeleke	65	s_adeleke@gamil.com	07056774423         	117	11
119	Lawal Tamire	35	l_tamire@gamil.com	09052111101         	107	5
120	James Job	44	j_job@gmail.com	08059693919         	100	8
121	Mattew Jakande	21	m_jakande@gmail.com	07051232144         	120	2
122	Jimila Adegboye	20	j_adegboye@gmail.com	08054921923         	107	5
\.


--
-- Name: customer customer_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.customer
    ADD CONSTRAINT customer_pkey PRIMARY KEY (c_id);


--
-- PostgreSQL database dump complete
--

\unrestrict PyhwZYsdwfke3ZxFpUeBo4dzKYCgtpjINqEMHBJ6NHmhdQoaL3Bj1nf1T1a4uWK

