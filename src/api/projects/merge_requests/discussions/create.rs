// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use chrono::{DateTime, Utc};
use derive_builder::Builder;

use crate::api::common::NameOrId;
use crate::api::endpoint_prelude::*;

/// A range of lines for a discussion.
#[derive(Debug, Clone, Builder)]
#[builder(setter(strip_option))]
pub struct LineRange<'a> {
    /// The line code for the start of the range.
    // FIXME: Add a type for this.
    #[builder(setter(into))]
    start_line_code: Cow<'a, str>,
    /// The line code for the end of the range.
    // FIXME: Add a type for this.
    #[builder(setter(into))]
    end_line_code: Cow<'a, str>,
}

impl<'a> LineRange<'a> {
    /// Create a builder for the line range.
    pub fn builder() -> LineRangeBuilder<'a> {
        LineRangeBuilder::default()
    }

    fn add_params<'b>(&'b self, params: &mut FormParams<'b>) {
        params
            .push(
                "position[line_range][start_line_code]",
                self.start_line_code.as_ref(),
            )
            .push(
                "position[line_range][end_line_code]",
                self.end_line_code.as_ref(),
            );
    }
}

/// A position within a text file for a discussion.
#[derive(Debug, Clone, Builder)]
#[builder(setter(strip_option))]
pub struct TextPosition<'a> {
    /// The name of the path for the new side of the diff.
    #[builder(setter(into), default)]
    new_path: Option<Cow<'a, str>>,
    /// The line number for the new side of the diff.
    #[builder(default)]
    new_line: Option<u64>,
    /// The name of the path for the old side of the diff.
    #[builder(setter(into), default)]
    old_path: Option<Cow<'a, str>>,
    /// The line number for the old side of the diff.
    #[builder(default)]
    old_line: Option<u64>,
    /// The range of lines to discuss.
    #[builder(default)]
    line_range: Option<LineRange<'a>>,
}

impl<'a> TextPosition<'a> {
    /// Create a builder for a text position.
    pub fn builder() -> TextPositionBuilder<'a> {
        TextPositionBuilder::default()
    }

    fn add_params<'b>(&'b self, params: &mut FormParams<'b>) {
        params
            .push_opt("position[new_path]", self.new_path.as_ref())
            .push_opt("position[new_line]", self.new_line)
            .push_opt("position[old_path]", self.old_path.as_ref())
            .push_opt("position[old_line]", self.old_line);

        if let Some(line_range) = self.line_range.as_ref() {
            line_range.add_params(params);
        }
    }
}

/// A position within an image for file a discussion.
#[derive(Debug, Clone, Copy, Builder)]
#[builder(setter(strip_option))]
pub struct ImagePosition {
    /// The width of the image.
    #[builder(default)]
    width: Option<u64>,
    /// The height of the image.
    #[builder(default)]
    height: Option<u64>,
    /// The `x` coordinate for the image.
    #[builder(default)]
    x: Option<u64>,
    /// The `y` coordinate for the image.
    #[builder(default)]
    y: Option<u64>,
}

impl ImagePosition {
    /// Create a builder for a image position.
    pub fn builder() -> ImagePositionBuilder {
        ImagePositionBuilder::default()
    }

    fn add_params<'b>(&'b self, params: &mut FormParams<'b>) {
        params
            .push_opt("position[width]", self.width)
            .push_opt("position[height]", self.height)
            .push_opt("position[x]", self.x)
            .push_opt("position[y]", self.y);
    }
}

#[derive(Debug, Clone)]
enum FilePosition<'a> {
    Text(TextPosition<'a>),
    Image(ImagePosition),
}

impl<'a> FilePosition<'a> {
    fn type_str(&self) -> &'static str {
        match self {
            Self::Text(_) => "text",
            Self::Image(_) => "image",
        }
    }

    fn add_params<'b>(&'b self, params: &mut FormParams<'b>) {
        match self {
            Self::Text(text) => text.add_params(params),
            Self::Image(image) => image.add_params(params),
        }
    }
}

/// A position in a merge request diff for a discussion.
#[derive(Debug, Clone, Builder)]
#[builder(setter(strip_option))]
pub struct Position<'a> {
    /// Tbe base commit SHA in the source branch.
    #[builder(setter(into))]
    base_sha: Cow<'a, str>,
    /// Tbe base commit SHA in the target branch.
    #[builder(setter(into))]
    start_sha: Cow<'a, str>,
    /// Tbe commit SHA for the HEAD of the merge request.
    #[builder(setter(into))]
    head_sha: Cow<'a, str>,
    /// The position within the diff to discuss.
    #[builder(setter(name = "_position"), private)]
    position: FilePosition<'a>,
}

impl<'a> PositionBuilder<'a> {
    /// The position within a text file.
    pub fn text_position(&mut self, position: TextPosition<'a>) -> &mut Self {
        self.position = Some(FilePosition::Text(position));
        self
    }

    /// The position within an image file.
    pub fn image_position(&mut self, position: ImagePosition) -> &mut Self {
        self.position = Some(FilePosition::Image(position));
        self
    }
}

impl<'a> Position<'a> {
    /// Create a builder for a position.
    pub fn builder() -> PositionBuilder<'a> {
        PositionBuilder::default()
    }

    fn add_params<'b>(&'b self, params: &mut FormParams<'b>) {
        params
            .push("position[base_sha]", self.base_sha.as_ref())
            .push("position[start_sha]", self.start_sha.as_ref())
            .push("position[head_sha]", self.head_sha.as_ref())
            .push("position[position_type]", self.position.type_str());

        self.position.add_params(params);
    }
}

/// Create a new discussion on a merge request on a project.
#[derive(Debug, Builder)]
#[builder(setter(strip_option))]
pub struct CreateMergeRequestDiscussion<'a> {
    /// The project of the merge request.
    #[builder(setter(into))]
    project: NameOrId<'a>,
    /// The merge method to start a new discussion on.
    merge_request: u64,
    /// The content of the discussion.
    #[builder(setter(into))]
    body: Cow<'a, str>,

    /// When the discussion was created.
    ///
    /// Requires administrator or owner permissions.
    #[builder(default)]
    created_at: Option<DateTime<Utc>>,
    /// The location of the discussion in the diff.
    #[builder(default)]
    position: Option<Position<'a>>,
}

impl<'a> CreateMergeRequestDiscussion<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> CreateMergeRequestDiscussionBuilder<'a> {
        CreateMergeRequestDiscussionBuilder::default()
    }
}

impl<'a> Endpoint for CreateMergeRequestDiscussion<'a> {
    fn method(&self) -> Method {
        Method::POST
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!(
            "projects/{}/merge_requests/{}/discussions",
            self.project, self.merge_request,
        )
        .into()
    }

    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, BodyError> {
        let mut params = FormParams::default();

        params
            .push("body", self.body.as_ref())
            .push_opt("created_at", self.created_at);

        if let Some(position) = self.position.as_ref() {
            position.add_params(&mut params);
        }

        params.into_body()
    }
}

#[cfg(test)]
mod tests {
    use chrono::{TimeZone, Utc};
    use http::Method;

    use crate::api::projects::merge_requests::discussions::{
        CreateMergeRequestDiscussion, ImagePosition, LineRange, Position, TextPosition,
    };
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    use super::FilePosition;

    #[test]
    fn line_range_start_and_end_are_necessary() {
        let err = LineRange::builder().build().unwrap_err();
        assert_eq!(err, "`start_line_code` must be initialized");
    }

    #[test]
    fn line_range_start_is_necessary() {
        let err = LineRange::builder()
            .end_line_code("end")
            .build()
            .unwrap_err();
        assert_eq!(err, "`start_line_code` must be initialized");
    }

    #[test]
    fn line_range_end_is_necessary() {
        let err = LineRange::builder()
            .start_line_code("start")
            .build()
            .unwrap_err();
        assert_eq!(err, "`end_line_code` must be initialized");
    }

    #[test]
    fn line_range_start_and_end_are_sufficient() {
        LineRange::builder()
            .start_line_code("start")
            .end_line_code("end")
            .build()
            .unwrap();
    }

    #[test]
    fn text_position_defaults_are_sufficient() {
        TextPosition::builder().build().unwrap();
    }

    #[test]
    fn image_position_defaults_are_sufficient() {
        ImagePosition::builder().build().unwrap();
    }

    #[test]
    fn file_position_type_str() {
        let items = &[
            (
                FilePosition::Text(TextPosition::builder().build().unwrap()),
                "text",
            ),
            (
                FilePosition::Image(ImagePosition::builder().build().unwrap()),
                "image",
            ),
        ];

        for (i, s) in items {
            assert_eq!(i.type_str(), *s);
        }
    }

    #[test]
    fn position_base_start_head_and_position_are_necessary() {
        let err = Position::builder().build().unwrap_err();
        assert_eq!(err, "`base_sha` must be initialized");
    }

    #[test]
    fn position_base_sha_is_necessary() {
        let err = Position::builder()
            .start_sha("deadbeefdeadbeefdeadbeefdeadbeefdeadbeef")
            .head_sha("cafebabecafebabecafebabecafebabecafebabe")
            .text_position(TextPosition::builder().build().unwrap())
            .build()
            .unwrap_err();
        assert_eq!(err, "`base_sha` must be initialized");
    }

    #[test]
    fn position_start_sha_is_necessary() {
        let err = Position::builder()
            .base_sha("0000000000000000000000000000000000000000")
            .head_sha("cafebabecafebabecafebabecafebabecafebabe")
            .text_position(TextPosition::builder().build().unwrap())
            .build()
            .unwrap_err();
        assert_eq!(err, "`start_sha` must be initialized");
    }

    #[test]
    fn position_head_sha_is_necessary() {
        let err = Position::builder()
            .base_sha("0000000000000000000000000000000000000000")
            .start_sha("deadbeefdeadbeefdeadbeefdeadbeefdeadbeef")
            .text_position(TextPosition::builder().build().unwrap())
            .build()
            .unwrap_err();
        assert_eq!(err, "`head_sha` must be initialized");
    }

    #[test]
    fn position_position_is_necessary() {
        let err = Position::builder()
            .base_sha("0000000000000000000000000000000000000000")
            .start_sha("deadbeefdeadbeefdeadbeefdeadbeefdeadbeef")
            .head_sha("cafebabecafebabecafebabecafebabecafebabe")
            .build()
            .unwrap_err();
        assert_eq!(err, "`position` must be initialized");
    }

    #[test]
    fn position_base_start_head_and_position_are_sufficient() {
        Position::builder()
            .base_sha("0000000000000000000000000000000000000000")
            .start_sha("deadbeefdeadbeefdeadbeefdeadbeefdeadbeef")
            .head_sha("cafebabecafebabecafebabecafebabecafebabe")
            .text_position(TextPosition::builder().build().unwrap())
            .build()
            .unwrap();
    }

    #[test]
    fn project_merge_request_and_body_are_necessary() {
        let err = CreateMergeRequestDiscussion::builder().build().unwrap_err();
        assert_eq!(err, "`project` must be initialized");
    }

    #[test]
    fn project_is_necessary() {
        let err = CreateMergeRequestDiscussion::builder()
            .merge_request(1)
            .body("body")
            .build()
            .unwrap_err();
        assert_eq!(err, "`project` must be initialized");
    }

    #[test]
    fn merge_request_is_necessary() {
        let err = CreateMergeRequestDiscussion::builder()
            .project(1)
            .body("body")
            .build()
            .unwrap_err();
        assert_eq!(err, "`merge_request` must be initialized");
    }

    #[test]
    fn body_is_necessary() {
        let err = CreateMergeRequestDiscussion::builder()
            .project(1)
            .merge_request(1)
            .build()
            .unwrap_err();
        assert_eq!(err, "`body` must be initialized");
    }

    #[test]
    fn project_merge_request_and_body_are_sufficient() {
        CreateMergeRequestDiscussion::builder()
            .project(1)
            .merge_request(1)
            .body("body")
            .build()
            .unwrap();
    }

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("projects/simple%2Fproject/merge_requests/1/discussions")
            .content_type("application/x-www-form-urlencoded")
            .body_str("body=body")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateMergeRequestDiscussion::builder()
            .project("simple/project")
            .merge_request(1)
            .body("body")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_created_at() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("projects/simple%2Fproject/merge_requests/1/discussions")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!("body=body", "&created_at=2020-01-01T00%3A00%3A00Z"))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateMergeRequestDiscussion::builder()
            .project("simple/project")
            .merge_request(1)
            .body("body")
            .created_at(Utc.ymd(2020, 1, 1).and_hms_milli(0, 0, 0, 0))
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_position_file() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("projects/simple%2Fproject/merge_requests/1/discussions")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!(
                "body=body",
                "&position%5Bbase_sha%5D=0000000000000000000000000000000000000000",
                "&position%5Bstart_sha%5D=deadbeefdeadbeefdeadbeefdeadbeefdeadbeef",
                "&position%5Bhead_sha%5D=cafebabecafebabecafebabecafebabecafebabe",
                "&position%5Bposition_type%5D=text",
            ))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateMergeRequestDiscussion::builder()
            .project("simple/project")
            .merge_request(1)
            .body("body")
            .position(
                Position::builder()
                    .base_sha("0000000000000000000000000000000000000000")
                    .start_sha("deadbeefdeadbeefdeadbeefdeadbeefdeadbeef")
                    .head_sha("cafebabecafebabecafebabecafebabecafebabe")
                    .text_position(TextPosition::builder().build().unwrap())
                    .build()
                    .unwrap(),
            )
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_position_file_full() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("projects/simple%2Fproject/merge_requests/1/discussions")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!(
                "body=body",
                "&position%5Bbase_sha%5D=0000000000000000000000000000000000000000",
                "&position%5Bstart_sha%5D=deadbeefdeadbeefdeadbeefdeadbeefdeadbeef",
                "&position%5Bhead_sha%5D=cafebabecafebabecafebabecafebabecafebabe",
                "&position%5Bposition_type%5D=text",
                "&position%5Bnew_path%5D=path%2Fto%2Ffile%2Fnew",
                "&position%5Bnew_line%5D=0",
                "&position%5Bold_path%5D=path%2Fto%2Ffile%2Fold",
                "&position%5Bold_line%5D=0",
                "&position%5Bline_range%5D%5Bstart_line_code%5D=some_complicated_line_code_thing",
                "&position%5Bline_range%5D%5Bend_line_code%5D=some_complicated_line_code_thing",
            ))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateMergeRequestDiscussion::builder()
            .project("simple/project")
            .merge_request(1)
            .body("body")
            .position(
                Position::builder()
                    .base_sha("0000000000000000000000000000000000000000")
                    .start_sha("deadbeefdeadbeefdeadbeefdeadbeefdeadbeef")
                    .head_sha("cafebabecafebabecafebabecafebabecafebabe")
                    .text_position(
                        TextPosition::builder()
                            .new_path("path/to/file/new")
                            .new_line(0)
                            .old_path("path/to/file/old")
                            .old_line(0)
                            .line_range(
                                LineRange::builder()
                                    .start_line_code("some_complicated_line_code_thing")
                                    .end_line_code("some_complicated_line_code_thing")
                                    .build()
                                    .unwrap(),
                            )
                            .build()
                            .unwrap(),
                    )
                    .build()
                    .unwrap(),
            )
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_position_image() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("projects/simple%2Fproject/merge_requests/1/discussions")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!(
                "body=body",
                "&position%5Bbase_sha%5D=0000000000000000000000000000000000000000",
                "&position%5Bstart_sha%5D=deadbeefdeadbeefdeadbeefdeadbeefdeadbeef",
                "&position%5Bhead_sha%5D=cafebabecafebabecafebabecafebabecafebabe",
                "&position%5Bposition_type%5D=image",
            ))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateMergeRequestDiscussion::builder()
            .project("simple/project")
            .merge_request(1)
            .body("body")
            .position(
                Position::builder()
                    .base_sha("0000000000000000000000000000000000000000")
                    .start_sha("deadbeefdeadbeefdeadbeefdeadbeefdeadbeef")
                    .head_sha("cafebabecafebabecafebabecafebabecafebabe")
                    .image_position(ImagePosition::builder().build().unwrap())
                    .build()
                    .unwrap(),
            )
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_position_image_full() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::POST)
            .endpoint("projects/simple%2Fproject/merge_requests/1/discussions")
            .content_type("application/x-www-form-urlencoded")
            .body_str(concat!(
                "body=body",
                "&position%5Bbase_sha%5D=0000000000000000000000000000000000000000",
                "&position%5Bstart_sha%5D=deadbeefdeadbeefdeadbeefdeadbeefdeadbeef",
                "&position%5Bhead_sha%5D=cafebabecafebabecafebabecafebabecafebabe",
                "&position%5Bposition_type%5D=image",
                "&position%5Bwidth%5D=100",
                "&position%5Bheight%5D=100",
                "&position%5Bx%5D=0",
                "&position%5By%5D=0",
            ))
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = CreateMergeRequestDiscussion::builder()
            .project("simple/project")
            .merge_request(1)
            .body("body")
            .position(
                Position::builder()
                    .base_sha("0000000000000000000000000000000000000000")
                    .start_sha("deadbeefdeadbeefdeadbeefdeadbeefdeadbeef")
                    .head_sha("cafebabecafebabecafebabecafebabecafebabe")
                    .image_position(
                        ImagePosition::builder()
                            .width(100)
                            .height(100)
                            .x(0)
                            .y(0)
                            .build()
                            .unwrap(),
                    )
                    .build()
                    .unwrap(),
            )
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }
}