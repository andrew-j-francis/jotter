<script lang="ts">
    import "carbon-components-svelte/css/g90.css";
    import {
        Button, Column, Content, FileUploader,
        Form,
        FormGroup, Grid,
        Header, Row,
        SideNav, SideNavItems, SideNavLink,
        TextInput
    } from "carbon-components-svelte";

    let isSideNavOpen = false;

    function handleSubmit() {

    }
</script>

<style>
    .formInput {
        margin: 1.5rem;
    }
</style>

<main>
    <Header platformName="Jotter - The JWT Generator for Salesforce OCI" bind:isSideNavOpen></Header>
    <SideNav bind:isOpen={isSideNavOpen}>
        <SideNavItems>
            <SideNavLink text="Help Quip"/>
            <SideNavLink text="Github Repo"/>
        </SideNavItems>
    </SideNav>


    <Content>
        <Grid>
            <Row>
                <Column>
                    <Form>
                        <FormGroup>
                            <div class="formInput">
                                <TextInput labelText="Connected App Consumer Key"
                                           helperText="Consumer Key related to your Connected App"></TextInput>
                            </div>
                            <div class="formInput">
                                <TextInput labelText="Org Username"
                                           placeholder="john.smith@universal-containers.com"></TextInput>
                            </div>
                            <div class="formInput">
                                <FileUploader buttonLabel="Upload PEM File"></FileUploader>
                            </div>
                        </FormGroup>
                        <div class="formInput">
                            <Button type="submit" on:click={handleSubmit}>Generate JWT</Button>
                        </div>
                    </Form>
                </Column>
            </Row>
        </Grid>
    </Content>

    <hr>

</main>