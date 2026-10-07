#[doc = "Register `FSCLR` reader"]
pub type R = crate::R<FsclrSpec>;
#[doc = "Register `FSCLR` writer"]
pub type W = crate::W<FsclrSpec>;
#[doc = "Field `PEWERRCLR` reader - desc PEWERRCLR"]
pub type PewerrclrR = crate::BitReader;
#[doc = "Field `PEWERRCLR` writer - desc PEWERRCLR"]
pub type PewerrclrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PEPRTERRCLR` reader - desc PEPRTERRCLR"]
pub type PeprterrclrR = crate::BitReader;
#[doc = "Field `PEPRTERRCLR` writer - desc PEPRTERRCLR"]
pub type PeprterrclrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PGSZERRCLR` reader - desc PGSZERRCLR"]
pub type PgszerrclrR = crate::BitReader;
#[doc = "Field `PGSZERRCLR` writer - desc PGSZERRCLR"]
pub type PgszerrclrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PGMISMTCHCLR` reader - desc PGMISMTCHCLR"]
pub type PgmismtchclrR = crate::BitReader;
#[doc = "Field `PGMISMTCHCLR` writer - desc PGMISMTCHCLR"]
pub type PgmismtchclrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OPTENDCLR` reader - desc OPTENDCLR"]
pub type OptendclrR = crate::BitReader;
#[doc = "Field `OPTENDCLR` writer - desc OPTENDCLR"]
pub type OptendclrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COLERRCLR` reader - desc COLERRCLR"]
pub type ColerrclrR = crate::BitReader;
#[doc = "Field `COLERRCLR` writer - desc COLERRCLR"]
pub type ColerrclrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc PEWERRCLR"]
    #[inline(always)]
    pub fn pewerrclr(&self) -> PewerrclrR {
        PewerrclrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc PEPRTERRCLR"]
    #[inline(always)]
    pub fn peprterrclr(&self) -> PeprterrclrR {
        PeprterrclrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc PGSZERRCLR"]
    #[inline(always)]
    pub fn pgszerrclr(&self) -> PgszerrclrR {
        PgszerrclrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc PGMISMTCHCLR"]
    #[inline(always)]
    pub fn pgmismtchclr(&self) -> PgmismtchclrR {
        PgmismtchclrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc OPTENDCLR"]
    #[inline(always)]
    pub fn optendclr(&self) -> OptendclrR {
        OptendclrR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc COLERRCLR"]
    #[inline(always)]
    pub fn colerrclr(&self) -> ColerrclrR {
        ColerrclrR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc PEWERRCLR"]
    #[inline(always)]
    pub fn pewerrclr(&mut self) -> PewerrclrW<'_, FsclrSpec> {
        PewerrclrW::new(self, 0)
    }
    #[doc = "Bit 1 - desc PEPRTERRCLR"]
    #[inline(always)]
    pub fn peprterrclr(&mut self) -> PeprterrclrW<'_, FsclrSpec> {
        PeprterrclrW::new(self, 1)
    }
    #[doc = "Bit 2 - desc PGSZERRCLR"]
    #[inline(always)]
    pub fn pgszerrclr(&mut self) -> PgszerrclrW<'_, FsclrSpec> {
        PgszerrclrW::new(self, 2)
    }
    #[doc = "Bit 3 - desc PGMISMTCHCLR"]
    #[inline(always)]
    pub fn pgmismtchclr(&mut self) -> PgmismtchclrW<'_, FsclrSpec> {
        PgmismtchclrW::new(self, 3)
    }
    #[doc = "Bit 4 - desc OPTENDCLR"]
    #[inline(always)]
    pub fn optendclr(&mut self) -> OptendclrW<'_, FsclrSpec> {
        OptendclrW::new(self, 4)
    }
    #[doc = "Bit 5 - desc COLERRCLR"]
    #[inline(always)]
    pub fn colerrclr(&mut self) -> ColerrclrW<'_, FsclrSpec> {
        ColerrclrW::new(self, 5)
    }
}
#[doc = "desc FSCLR\n\nYou can [`read`](crate::Reg::read) this register and get [`fsclr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fsclr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FsclrSpec;
impl crate::RegisterSpec for FsclrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fsclr::R`](R) reader structure"]
impl crate::Readable for FsclrSpec {}
#[doc = "`write(|w| ..)` method takes [`fsclr::W`](W) writer structure"]
impl crate::Writable for FsclrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FSCLR to value 0"]
impl crate::Resettable for FsclrSpec {}
