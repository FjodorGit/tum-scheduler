use std::env;

use reqwest::Client;

use super::ScraperError;

#[derive(Debug)]
pub struct CourseDescription(pub String);

#[derive(Debug)]
pub struct CourseDescriptionEndpoint {
    pub client: Client,
    pub course_description_url: String,
    pub semester_filter_url: String,
}

impl CourseDescription {
    pub fn empty() -> Self {
        Self("No Description".to_string())
    }
}

impl CourseDescriptionEndpoint {
    pub fn for_semester(semester_id: &str) -> Self {
        let base_request_url = env::var("DESCRIPTION_LIST_URL")
            .expect("DESCRIPTION_LIST_URL should exist in environment variables");
        let semester_filter_url = format!("{}&pFilterSemesterNr={}", base_request_url, semester_id);
        let course_description_url = env::var("DESCRIPTION_URL")
            .expect("DESCRIPTION_URL should exist in environment variables");
        let client = reqwest::Client::new();
        Self {
            client,
            course_description_url,
            semester_filter_url,
        }
    }

    pub async fn get_subject_description(
        &self,
        subject: &str,
    ) -> Result<CourseDescription, ScraperError> {
        let course_filter_url = format!(
            "{}&pFilterNameOrKennung={}",
            self.semester_filter_url, subject
        );
        let course_list_response = self
            .client
            .get(course_filter_url)
            .send()
            .await?
            .text()
            .await?;
        let knoten_nr = Self::get_knoten_nr(course_list_response)?;
        let course_description_url =
            format!("{}&pKnotenNr={}", self.course_description_url, knoten_nr);
        // println!("Course description url: {:#?}", course_description_url);
        let course_description = self
            .client
            .get(course_description_url)
            .send()
            .await?
            .text()
            .await?;
        let course_content = Self::get_content(course_description)?;

        Ok(CourseDescription(course_content))
    }

    pub async fn get_subjects_description<T: Iterator<Item = String>>(
        &self,
        subjects: T,
    ) -> Result<Vec<CourseDescription>, ScraperError> {
        let mut descriptions: Vec<CourseDescription> = vec![];
        for subject in subjects {
            let description = self.get_subject_description(&subject).await?;
            descriptions.push(description);
        }
        Ok(descriptions)
    }

    fn get_text_after_first(
        text_to_search: String,
        word_to_find: &str,
    ) -> Result<String, ScraperError> {
        let text_after = text_to_search
            .chars()
            .skip(
                text_to_search
                    .find(word_to_find)
                    .ok_or(ScraperError::DocumentParseError(format!(
                        "Could not find {} in document",
                        word_to_find
                    )))?,
            )
            .skip(word_to_find.len())
            .collect::<String>();
        Ok(text_after)
    }

    fn remove_special_whitespaces(text: &mut String) {
        for pattern in &["\t", "\r", "\n"] {
            *text = text.replace(pattern, "");
        }
    }

    pub fn get_content(response_text: String) -> Result<String, ScraperError> {
        let text_after_content = Self::get_text_after_first(response_text, "Content<")?;
        let content_start = Self::get_text_after_first(text_after_content, "MaskRenderer\">")?;
        let end_of_content_index =
            content_start
                .find("</td>")
                .ok_or(ScraperError::DocumentParseError(
                    "Could not find </td> in document".to_string(),
                ))?;
        let mut content_text = content_start
            .chars()
            .take(end_of_content_index)
            .collect::<String>();
        // println!("Content_text: {:#?}", content_text);
        Self::remove_special_whitespaces(&mut content_text);
        Ok(content_text)
    }

    pub fn get_knoten_nr(response_text: String) -> Result<String, ScraperError> {
        let knoten_nr = Self::get_text_after_first(response_text, "pKnotenNr=")?
            .chars()
            .take_while(|c| c.is_ascii_digit())
            .collect::<String>();
        Ok(knoten_nr)
    }
}

#[cfg(test)]
mod test {

    use dotenv::dotenv;
    use std::fs;

    use super::CourseDescriptionEndpoint;

    #[test]
    fn test_getting_knoten_nr() {
        let test_string = r"n_inactive.gif?20130606222106' width='14'></a></span><span><a href='#' class='headerTool nounderline' id='idFilterA28430093735' onclick='try{return(false);}catch(e){return($E(e));}' title='ungefiltert, klick um Filter anzuzeigen'><img alt='' height='14' src='/tumprod/img/tableheader_filter_inactive.gif?20101207172149' width='14'></a></span></div> </th> <th colspan='1' class=' L'> <span class='TextToolTip ' title='Versionskurzbezeichnung'>Vers.</span></th> <th colspan='1' class=' L' id='idCOTH28430093736'> <div> <span><span class='TextToolTip ' title='Betreuende Organisation der MHB-Vorlage'>Org. Kenn.</span></span><span><span class='headerToolSpacer '>&nbsp;</span></span><span><a href='#' class='headerTool nounderline' onclick='try{CO.Table.goTo($(&quot;idModHBTableORG&quot;), null, null, true, &quot;4&quot;); return(false);}catch(e){return($E(e));}' title='unsortiert, klick um aufsteigend zu sortieren'><img alt='' height='14' src='/tumprod/img/tableheader_sort_down_inactive.gif?20130606222106' width='14'></a></span><span><a href='#' class='headerTool nounderline' id='idFilterA28430093738' onclick='try{return(false);}catch(e){return($E(e));}' title='ungefiltert, klick um Filter anzuzeigen'><img alt='' height='14' src='/tumprod/img/tableheader_filter_inactive.gif?20101207172149' width='14'></a></span></div> </th> </tr> </thead> <tfoot> <tr> <th colspan='4' class='ftr'> &nbsp; <span id='idTimer28430093693'></span></th> </tr> </tfoot> <tbody> <tr class='coRow z0 hi coTableR '> <td class='bold L'><a href='#' name='coTblIdx_idModHBTableORG_1' onblur='try{CO.Table.blurRow(this);}catch(e){return($E(e));}' onfocus='try{CO.Table._focusRow($(&quot;idModHBTableORG&quot;), this);}catch(e){return($E(e));}' class='coTabRA' id='coTblIdx_idModHBTableORG_1' onclick='try{ return(false);}catch(e){return($E(e));}'></a><a href='WBMODHB.wbShowMHBReadOnly?pKnotenNr=1151206&amp;pOrgNr=1' onclick='try{return(CO.OV(this,'Mini'));}catch(e){return($E(e));}'>Topologie</a> </td><td class='bold L'>MA3241</td><td class=' L'>v2</td><td class=' L'><span class='TextToolTip ' title='Ehemalige Fakult&auml;t f&uuml;r Mathematik'>TUMAFMA</span></td></tr> <tr class='coRow z1 hi coTableR '> <td class='bold L'><a href='#' name='coTblIdx_idModHBTableORG_2' onblur='try{CO.Table.blurRow(this);}catch(e){return($E(e));}' onfocus='t'".to_string();
        let knoten_nr = CourseDescriptionEndpoint::get_knoten_nr(test_string).unwrap();
        assert_eq!(knoten_nr, "1151206".to_string());
    }

    #[test]
    fn test_gettting_content() {
        let test_string = fs::read_to_string("test_xmls/knoten_response.txt")
            .expect("knoten_response file should be available");
        let content = CourseDescriptionEndpoint::get_content(test_string)
            .expect("should not error on content extraction");
        assert_eq!(content, "I) Representations of data as matrices</br>a. Many data vectors form a matrix</br>b. Review of basic linear algebra</br>c. Linear dependence and concept of rank</br>d. Approximate linear dependence with varying degree of approximation: Singular value decomposition /Principal Component Analysis </br>e. Redundancy of data representations -> orthonormal bases, frames and dictionaries</br>f. Fourier basis as singular vectors of spatial shift</br>g. Fast Fourier Transform </br>II) Linear dimension reduction</br>a. Johnson-Lindenstrauss (JL) Lemma</br>b. Review of basic probability, random matrices</br>c. Random Matrices satisfying JL with high probability</br>d. Fast JL embeddings</br>e. Sparsity, low rank as structured signal models</br>f. Compressed sensing</br>g. Matrix completion and low rank matrix recovery</br>h. Optimization review</br>j. Dictionary Learning</br>III) Non-linear dimension reduction</br>a. Manifolds as data models</br>b. Review of differential geometry</br>c. ISOMAP</br>d. Diffusion maps</br>e. Importance of Nearest neighbor search, use of JL</br>IV) Outlook: Data Analysis and Machine Learning")
    }

    #[tokio::test]
    async fn test_getting_course_description() {
        dotenv().ok();
        dotenv::from_filename("request_urls").ok();

        let subjects = vec!["CIT413026".to_string(), "MA3303".to_string()].into_iter();
        let course_description_endpoint = CourseDescriptionEndpoint::for_semester("200");
        let description = course_description_endpoint
            .get_subjects_description(subjects)
            .await
            .expect("should be able to fetch course content");
        assert_eq!(description[0].0, "- Topological Groups;</br>- Integration on Topological Groups and Convolution; </br></br>- Representation Theory of Topological Groups and related Group Algebras </br>- Harmonic Analysis on Abelian and Compact Groups;</br>- Harmonic Analysis on Homogeneuous Spaces and Double Coset Spaces ; Spherical Functions");
        assert_eq!(description[1].0, "Finite element methods for the discretization of (multidimensional) elliptic boundary value problems: a priori and a posteriori error analysis, adaptive mesh refinement, fast solvers. Introduction to numerical methods for evolution equations");
    }
}
